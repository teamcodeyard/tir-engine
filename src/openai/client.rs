use reqwest;
use serde::{Deserialize, Serialize};
use std::fmt::*;

const OPENAI_URL: &'static str = "https://api.openai.com/v1/chat/completions";

#[derive(Debug, Serialize)]
pub struct OpenAIRequestMessage {
    pub role: String,
    pub content: String,
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

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: ChoiceMessage,
}

#[derive(Deserialize, Debug)]
pub struct ChoiceMessage {
    pub role: String,
    pub content: String,
}

pub struct Client {
    pub secret_key: String,
}

impl Client {
    pub async fn get_ai_response(
        &self,
        messages: Vec<OpenAIRequestMessage>,
        max_tokens: u32,
    ) -> OpenAIResponse {
        let client = reqwest::Client::new();
        let request = OpenAIRequest {
            messages,
            max_tokens,
            model: "gpt-3.5-turbo".to_string(),
        };
        let response = client
            .post(OPENAI_URL)
            .header("Authorization", format!("Bearer {}", self.secret_key)) // Replace "YOUR_OPENAI_KEY" with your actual key
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .unwrap()
            .json::<OpenAIResponse>()
            .await;

        return response.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration;
    use tokio::test;

    #[test]
    async fn test_initialize_client() {
        let secret_key = String::from("VERY_SECRET_KEY");
        let client = Client {
            secret_key: secret_key.clone(),
        };
        assert_eq!(client.secret_key, secret_key);
    }

    #[test]
    async fn test_get_ai_response() {
        configuration::load_env(String::from(".env"));
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
        let client = Client {
            secret_key: secret_key.clone(),
        };
        let response = client.get_ai_response(
            vec![OpenAIRequestMessage {
                role: String::from("system"),
                content: String::from("2+2? Please send only the answer number"),
            }],
            1,
        ).await;
        assert_eq!(response.choices.len(), 1);
        let choice = response.choices.first();
        match choice {
            Some(_choice) => assert_eq!(_choice.message.content, "4"),
            None => assert!(false)
        }
    }


    #[test]
    #[should_panic(expected = "missing field `choices`")]
    async fn test_get_ai_response_negative() {
        configuration::load_env(String::from(".env"));
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
        let client = Client {
            secret_key: secret_key.clone(),
        };
        let _ = client.get_ai_response(
            vec![],
            0,
        ).await;
    }

    
}
