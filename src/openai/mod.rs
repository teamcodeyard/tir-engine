use self::client::{Client, OpenAIRequestMessage, OpenAIResponse};
use crate::structs::{AnswerResult, Thematic, Topic};
use regex::Regex;
mod client;

const GK_START_PROMPT: &str = "You are my mentor and also a senior software engineer.";
const EA_START_PROMPT: &str = "You are my mentor and also a senior software engineer.";
const CE_START_PROMPT: &str = "You are my mentor and also a senior software engineer.";

pub struct GPT {
    client: Client,
}

impl GPT {
    pub fn new(secret_key: String) -> Self {
        let client = Client { secret_key };

        Self { client }
    }

    async fn ask(&self, messages: Vec<OpenAIRequestMessage>) -> OpenAIResponse {
        return self.ask_with_limits(messages, 500).await; // TODO configurate it
    }

    async fn ask_with_limits(
        &self,
        messages: Vec<OpenAIRequestMessage>,
        max_tokens: u32,
    ) -> OpenAIResponse {
        return self.client.get_ai_response(messages, max_tokens).await;
    }

    pub async fn generate_knowledge(&self, thematic: &mut Thematic) {
        for mut topic in &mut thematic.topics {
            let messages: Vec<OpenAIRequestMessage> = vec![
                    OpenAIRequestMessage {
                        role:String::from("system"),
                        content: GK_START_PROMPT.to_string(),
                    },
                    OpenAIRequestMessage {
                        role: String::from("user"),
                        content: format!(
                            "Explain me the {} topic ({}) in 500 characters please, I am a very beginner in software development.",
                            topic.title, thematic.title
                        ),
                    },
                ];
            let response = self.ask(messages).await;
            topic.explanation = Some(response.choices.first().unwrap().message.content.clone());
        }
    }

    pub async fn evaluate_answer(&self, answer: String, topic: Topic) -> AnswerResult {
        let messages: Vec<OpenAIRequestMessage> = vec![
            OpenAIRequestMessage {
                role: String::from("system"),
                content: EA_START_PROMPT.to_string(),
            },
            OpenAIRequestMessage {
                role: String::from("user"),
                content: format!(
                    "You asked me to explain the '{}' topic, please rate my answer between 1-10 and put your score between two % sign (for example: I would rate your answer a %6% out of 10). My answer: '{}'",
                    topic.title, answer
                ),
            },
        ];
        let response = self.ask(messages).await;
        let result_text: &str =
            &Some(response.choices.first().unwrap().message.content.clone()).unwrap();
        let re = Regex::new(r"%(\d+)%").unwrap();
        let caps = re.captures(result_text).unwrap();
        let score: u8 = caps[1].parse().unwrap();
        let explanation = re.replace(result_text, &caps[1]);
        AnswerResult {
            score,
            explanation: explanation.to_string(),
        }
    }

    pub async fn correct_explanation(&self, correction: String, topic: &mut Topic) {
        let messages: Vec<OpenAIRequestMessage> = vec![
            OpenAIRequestMessage {
                role: String::from("system"),
                content: CE_START_PROMPT.to_string(),
            },
            OpenAIRequestMessage {
                role: String::from("user"),
                content: format!(
                    "For the '{}' topic explanation you gave me this answer: '{}'. Please correct it with this instruction: '{}'",
                    topic.title, topic.explanation.as_ref().unwrap(), correction
                ),
            },
        ];
        let response = self.ask(messages).await;
        topic.explanation = Some(response.choices.first().unwrap().message.content.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration;
    use tokio::test;

    #[test]
    async fn test_initialize() {
        let secret_key = String::from("VERY_SECRET_KEY");
        let gpt_client = GPT::new(secret_key.clone());
        assert_eq!(gpt_client.client.secret_key, secret_key);
    }

    #[test]
    async fn test_ask_response_length() {
        configuration::load_env(String::from(".env"));
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
        let gpt_client = GPT::new(secret_key.clone());
        let response = gpt_client
            .ask_with_limits(
                vec![OpenAIRequestMessage {
                    role: String::from("system"),
                    content: String::from("Write one word only"),
                }],
                1,
            )
            .await;
        let choice = response.choices.first().unwrap();
        let word_count = choice.message.content.split_ascii_whitespace().count();
        assert_eq!(word_count, 1);
    }

    #[test]
    async fn test_generate_knowledge() {
        configuration::load_env(String::from(".env"));
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
        println!("{:?}-",secret_key);
        let client = GPT::new(secret_key);
        let mut thematic = Thematic {
            title: String::from("Desing patterns"),
            topics: vec![Topic {
                title: String::from("Singleton"),
                explanation: Some(String::from("")),
            }],
        };

        client.generate_knowledge(&mut thematic).await;
        for topic in thematic.topics {
            assert!(topic.explanation.is_some());
        }
    }
    #[test]
    async fn correct_explanation() {
        configuration::load_env(String::from(".env"));
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
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
            .await;
        assert_ne!(topic.explanation.unwrap(), explanation);
    }

    #[test]
    async fn test_evaluate_answer() {
        configuration::load_env(String::from(".env"));
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
        let client = GPT::new(secret_key);

        let topic = Topic {
            title: String::from("Singleton design pattern"),
            explanation: Some(String::from("")),
        };

        let result = client.evaluate_answer(String::from("Singleton is a design pattern in the software development when you only have one instance from a Class."), topic).await;
        assert!((1..=10).contains(&result.score));
        assert!(result.explanation.len() > 5);
    }
}
