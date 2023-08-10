use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::error::{OpenAIError, TirError};

const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";
const GPT_35_TURBO: &str = "gpt-3.5-turbo";

#[derive(Debug, Serialize)]
pub struct OpenAIRequestMessage {
    // Technically both `role` and `content` can be an owned `String`, or a `&str` string reference.
    // The `std::borrow::Cow` type will ensure we don't allocate when it's not necessary.
    pub role: Cow<'static, str>,
    pub content: Cow<'static, str>,
}

impl OpenAIRequestMessage {
    // By accepting `impl Into<Cow<'static, str>>`, we may accept `String`s or `&str`s too.
    // This is just makes it more comfortable to construct this type.
    pub(crate) fn from_parts(
        role: impl Into<Cow<'static, str>>,
        content: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
        }
    }

    pub(crate) fn with_system_role(content: impl Into<Cow<'static, str>>) -> Self {
        Self {
            role: "system".into(),
            content: content.into(),
        }
    }

    pub(crate) fn with_user_role(content: impl Into<Cow<'static, str>>) -> Self {
        Self {
            role: "user".into(),
            content: content.into(),
        }
    }
}

#[derive(Serialize)]
struct OpenAIRequest {
    messages: Vec<OpenAIRequestMessage>,
    max_tokens: u32,
    model: &'static str,
}

#[derive(Deserialize, Debug)]
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
    // To avoid creating a new reqwest client on every call to `get_ai_response`, let's store it here.
    reqwest_client: reqwest::Client,
}

// A dirty trick to be able to deserialize one of two types relatively seamlessly.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl Client {
    pub fn new(secret_key: String) -> Self {
        let reqwest_client = reqwest::Client::new();
        Self {
            secret_key,
            reqwest_client,
        }
    }

    pub async fn get_ai_response(
        &self,
        messages: Vec<OpenAIRequestMessage>,
        max_tokens: u32,
    ) -> Result<OpenAIResponse, TirError> {
        let request = OpenAIRequest {
            messages,
            max_tokens,
            model: GPT_35_TURBO,
        };
        let response = self
            .reqwest_client
            .post(OPENAI_URL)
            .bearer_auth(&self.secret_key)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&request)
            .send()
            .await?.json::<Either<OpenAIResponse, OpenAIError>>()
            .await;

        // If we get back an error JSON from OpenAI, parse and keep that around in the `Err` variant.
        // We'll have better analytics, and we may save some debugging time too.
        match response {
            Ok(Either::Left(l)) => Ok(l),
            Ok(Either::Right(r)) => Err(TirError::OpenAIError(r)),
            Err(e) => Err(TirError::Request(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration;
    use tokio::test;

    #[test]
    async fn test_get_ai_response() {
        configuration::load_env(".env");
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
        let client = Client::new(secret_key);
        let response = client
            .get_ai_response(
                vec![OpenAIRequestMessage::with_system_role(
                    "2+2? Please send only the answer number",
                )],
                1,
            )
            .await
            .unwrap();
        assert_eq!(response.choices.len(), 1);
        assert_eq!(
            response
                .choices
                .first()
                .unwrap()
                .message
                .content
                .parse::<usize>()
                .unwrap(),
            4
        );
    }

    #[test]
    async fn test_get_ai_response_negative() {
        configuration::load_env(".env");
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
        let client = Client::new(secret_key);
        let result = client.get_ai_response(vec![], 0).await;
        assert!(matches!(
            result,
            Err(TirError::OpenAIError(_))
        ));
    }
}
