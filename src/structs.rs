
#[derive(Debug, serde::Deserialize, Clone)]
pub struct AnswerResult {
    pub score: u8,
    pub explanation: String,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct Topic {
    pub title: String,
    pub explanation: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Thematic {
    pub title: String,
    pub topics: Vec<Topic>,
}