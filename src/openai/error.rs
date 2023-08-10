use serde::{Deserialize, Serialize};

// A custom error enum. Not strictly necessary, but as you grow your app, more and more error variants will show up.
// The `thiserror` crate makes this pleasant to write and to group them.
// It this is too heavy, the `anyhow` create is also an option to consider (it's more often used for general "application" code)
#[derive(thiserror::Error, Debug)]
pub enum TirError {
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("OpenAI error: {0:?}")]
    OpenAIError(OpenAIError),
    #[error("choice vector was empty")]
    EmptyChoiceVec,
}

// An error JSON we may get back from OpenAI, for example:
//
//    {"error": {"code": "something", "message": "something else"}}
//
// Note: I haven't looked at OpenAI documentation to validate this scheme. Take this with a grain of salt.
#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIError {
    pub error: Option<OpenAiErrorDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiErrorDetails {
    pub code: Option<String>,
    pub message: Option<String>,
}
