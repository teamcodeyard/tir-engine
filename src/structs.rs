#[derive(Debug, serde::Deserialize, Clone, serde::Serialize)]
pub struct Answer {
    pub score: u8,
    pub explanation: String,
}
#[derive(Debug, serde::Deserialize, Clone, serde::Serialize)]
pub struct Topic {
    pub title: String,
    pub explanation: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Thematic {
    pub title: String,
    pub topics: Vec<Topic>,
}
