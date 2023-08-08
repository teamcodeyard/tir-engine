mod configuration;
mod openai;
mod structs;
use structs::{AnswerResult, Thematic, Topic};

fn get_client() -> openai::GPT {
    configuration::load_env(String::from(".env"));
    let secret_key = configuration::get_var("OPENAI_SK").unwrap();
    openai::GPT::new(secret_key)
}

pub async fn generate_knowledge() -> Vec<Thematic> {
    let client = get_client();
    let roadmap_file_path = configuration::get_var("ROADMAP_FILE_PATH").unwrap();
    let mut roadmap = configuration::load_roadmap(roadmap_file_path);
    for mut thematic in &mut roadmap {
        client.generate_knowledge(&mut thematic).await;
    }
    return roadmap;
}

pub async fn evaluate_answer(answer: String, topic: Topic) -> AnswerResult {
    let client = get_client();
    client.evaluate_answer(answer, topic).await
}

pub async fn correct_explanation(correction: String, mut topic: &mut Topic) {
    let client = get_client();
    client.correct_explanation(correction, &mut topic).await
}