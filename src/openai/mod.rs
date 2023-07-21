use crate::configuration::Roadmap;
mod client;
use self::client::{Client, OpenAIRequestMessage, OpenAIResponse};

const START_PROMPT: &str = "You are my mentor and also a senior software engineer.";

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

    pub async fn generate_knowledge(&self, roadmap: Vec<Roadmap>) -> OpenAIResponse {
        let thematic = &roadmap[1];
        let topic = thematic.topics.first().unwrap();
        let messages: Vec<OpenAIRequestMessage> = vec![
            OpenAIRequestMessage {
                role: "system".to_string(),
                content: START_PROMPT.to_string(),
            },
            OpenAIRequestMessage {
                role: "user".to_string(),
                content: format!(
                    "Explain me the {} topic ({}) in 500 characters please, I am a very beginner in software development.",
                    topic, thematic.title
                ),
            },
        ];
        self.ask(messages).await
    }
}
