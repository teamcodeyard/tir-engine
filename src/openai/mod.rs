use self::{
    client::{Client, OpenAIRequestMessage, OpenAIResponse},
    error::TirError,
};
use crate::structs::{Answer, Thematic, Topic};
use regex::Regex;
mod client;
pub mod error;
use std::sync::OnceLock;

const GK_START_PROMPT: &str = "You are my mentor and also a senior software engineer.";
const EA_START_PROMPT: &str = "You are my mentor and also a senior software engineer.";
const CE_START_PROMPT: &str = "You are my mentor and also a senior software engineer.";

// Regex compilation is expensive - let's do it only once. The first run of `GPT::evaluate_answer` will initialize this.
// For details: https://docs.rs/regex/latest/regex/#avoid-re-compiling-regexes-especially-in-a-loop
static RE: OnceLock<Regex> = OnceLock::new();

pub struct GPT {
    client: Client,
}

impl GPT {
    pub fn new(secret_key: String) -> Self {
        Self {
            client: Client::new(secret_key),
        }
    }

    async fn ask(&self, messages: Vec<OpenAIRequestMessage>) -> Result<OpenAIResponse, TirError> {
        self.ask_with_limits(messages, 500).await // TODO configurate it
    }

    async fn ask_with_limits(
        &self,
        messages: Vec<OpenAIRequestMessage>,
        max_tokens: u32,
    ) -> Result<OpenAIResponse, TirError> {
        self.client.get_ai_response(messages, max_tokens).await
    }

    pub async fn generate_knowledge(&self, thematic: &mut Thematic) -> Result<(), TirError> {
        for topic in &mut thematic.topics {
            let messages: Vec<OpenAIRequestMessage> = vec![
                OpenAIRequestMessage::with_system_role(GK_START_PROMPT),
                OpenAIRequestMessage::with_user_role(format!(
                        "Explain me the {} topic ({}) in 500 characters please, I am a very beginner in software development.",
                        topic.title, thematic.title
                    ))
            ];
            let response = self.ask(messages).await?;
            // If there's no first element in `choices`, immediately bubble up with an error.
            // Feel free to change this: I don't know whether this is the right thing to do based on business logic.
            // `unwrap`ing however should definitely be avoided.
            //
            // Note: if you'd like to set the `explanation` to `None` if there was nothing in `choices`, you may write:
            //
            // topic.explanation = response
            //     .choices
            //     .first()
            //     .map(|choice| choice.message.content.clone())
            topic.explanation = Some(
                response
                    .choices
                    .first()
                    .ok_or(TirError::EmptyChoiceVec)?
                    .message
                    .content
                    .clone(),
            );
        }
        Ok(())
    }

    pub async fn evaluate_answer(&self, answer: String, topic: Topic) -> Result<Answer, TirError> {
        let messages: Vec<OpenAIRequestMessage> = vec![
            OpenAIRequestMessage::with_system_role(EA_START_PROMPT),
            OpenAIRequestMessage::with_user_role(format!(
                    "You asked me to explain the '{}' topic, please rate my answer between 1-10 and put your score between two % sign (for example: I would rate your answer a %6% out of 10). My answer: '{}'",
                    topic.title, answer
                ))
        ];
        let response = self.ask(messages).await?;
        let result_text = &response
            .choices
            .first()
            .ok_or(TirError::EmptyChoiceVec)?
            .message
            .content;
        let caps = RE
            .get_or_init(|| Regex::new(r"%(\d+)%").unwrap())
            .captures(result_text)
            .unwrap();
        let score: u8 = caps[1].parse().unwrap();
        let explanation = RE
            .get()
            .expect("initialized above")
            .replace(result_text, &caps[1]);
        Ok(Answer {
            score,
            explanation: explanation.to_string(),
        })
    }

    pub async fn correct_explanation(
        &self,
        correction: String,
        topic: &mut Topic,
    ) -> Result<(), TirError> {
        let messages: Vec<OpenAIRequestMessage> = vec![
            OpenAIRequestMessage::with_system_role(CE_START_PROMPT),
            OpenAIRequestMessage::with_user_role(format!(
                    "For the '{}' topic explanation you gave me this answer: '{}'. Please correct it with this instruction: '{}'",
                    topic.title, topic.explanation.as_ref().unwrap(), correction
                ))
        ];
        let response = self.ask(messages).await?;
        topic.explanation = Some(
            response
                .choices
                .first()
                .ok_or(TirError::EmptyChoiceVec)?
                .message
                .content
                .clone(),
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_initialize() {
        let secret_key = String::from("VERY_SECRET_KEY");
        let gpt_client = GPT::new(secret_key.clone());
        assert_eq!(gpt_client.client.secret_key, secret_key);
    }

    #[test]
    async fn test_ask_response_length() {
        let secret_key = std::env::var("OPENAI_SK").unwrap();
        let gpt_client = GPT::new(secret_key.clone());
        let response = gpt_client
            .ask_with_limits(
                vec![OpenAIRequestMessage::with_system_role(
                    "Write one word only",
                )],
                1,
            )
            .await
            .unwrap();
        let choice = response.choices.first().unwrap();
        let word_count = choice.message.content.split_ascii_whitespace().count();
        assert_eq!(word_count, 1);
    }

    #[test]
    async fn test_generate_knowledge() {
        let secret_key = std::env::var("OPENAI_SK").unwrap();
        let client = GPT::new(secret_key);
        let mut thematic = Thematic {
            title: String::from("Design patterns"),
            topics: vec![Topic {
                title: String::from("Singleton"),
                explanation: Some(String::from("")),
            }],
        };

        client.generate_knowledge(&mut thematic).await.unwrap();
        for topic in thematic.topics {
            assert!(topic.explanation.is_some());
        }
    }
    #[test]
    async fn correct_explanation() {
        let secret_key = std::env::var("OPENAI_SK").unwrap();
        let client = GPT::new(secret_key);
        let explanation = String::from("The Singleton pattern is a design pattern that ensures only one instance of a class is created throughout the application. It is useful when you want to restrict the instantiation of a class to a single object and ensure that no other object can create additional instances.");
        let mut topic = Topic {
            title: String::from("Singleton design pattern"),
            explanation: Some(explanation.clone()),
        };

        client
            .correct_explanation(
                String::from(
                    "Please extend your answer with global access point to the singleton instance",
                ),
                &mut topic,
            )
            .await
            .unwrap();
        assert_ne!(topic.explanation.unwrap(), explanation);
    }

    #[test]
    async fn test_evaluate_answer() {
        let secret_key = std::env::var("OPENAI_SK").unwrap();
        let client = GPT::new(secret_key);

        let topic = Topic {
            title: String::from("Singleton design pattern"),
            explanation: Some(String::from("")),
        };

        let result = client.evaluate_answer(String::from("Singleton is a design pattern in the software development when you only have one instance from a Class."), topic).await.unwrap();
        assert!((1..=10).contains(&result.score));
        assert!(result.explanation.len() > 5);
    }
}
