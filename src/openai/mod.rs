use self::client::{Client, OpenAIRequestMessage, OpenAIResponse};
use crate::structs::{AnswerResult, Thematic, Topic};
use regex::Regex;
mod client;


const GK_START_PROMPT: &str = "You are my mentor and also a senior software engineer.";
const EA_START_PROMPT: &str = "You are my mentor and also a senior software engineer.";
const CE_START_PROMPT: &str = "You are my mentor and also a senior software engineer.";

pub struct GPT {
    pub client: Client,
}

impl GPT {
    pub fn new(secret_key: String) -> Self {
        let client = Client { secret_key };

        Self { client }
    }

    async fn ask(&self, messages: Vec<OpenAIRequestMessage>) -> OpenAIResponse {
        println!("{:?}", messages);
        return self.client.get_ai_response(messages, 500).await;
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
