use reqwest;
use serde::{Deserialize, Serialize};
use std::fmt::*;

const OPENAI_URL: &'static str = "https://api.openai.com/v1/chat/completions";


#[derive(Debug, Serialize)]
pub struct OpenAIRequestMessage {
    pub role: String,
    pub content: String
}

#[derive(Serialize)]
struct OpenAIRequest {
    messages: Vec<OpenAIRequestMessage>,
    max_tokens: u32,
    model: String,
}

#[derive(Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<Choice>,
}


#[derive(Deserialize)]
pub struct Choice {
    pub message: ChoiceMessage,
}

#[derive(Deserialize)]
pub struct ChoiceMessage {
    pub role: String,
    pub content: String,
}

pub struct Client {
    pub secret_key: String,
}

impl Client {
    pub async fn get_ai_response(&self, messages: Vec<OpenAIRequestMessage>, max_tokens: u32) -> OpenAIResponse {
        let client = reqwest::Client::new();
        let request = OpenAIRequest { messages, max_tokens, model: "gpt-3.5-turbo".to_string(), };
    
        let response = client
            .post(OPENAI_URL)
            .header("Authorization", format!("Bearer {}", self.secret_key)) // Replace "YOUR_OPENAI_KEY" with your actual key
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await.unwrap()
            .json::<OpenAIResponse>()
            .await;
    
        return response.unwrap()
    }
    
}