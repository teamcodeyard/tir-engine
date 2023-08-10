mod configuration;
mod openai;
mod structs;
use openai::error::TirError;
use structs::{Answer, Thematic, Topic};

fn get_client() -> openai::GPT {
    configuration::load_env(".env");
    let secret_key = configuration::get_var("OPENAI_SK").unwrap();
    openai::GPT::new(secret_key)
}

pub async fn generate_knowledge() -> Result<Vec<Thematic>, TirError> {
    let client = get_client();
    let roadmap_file_path = configuration::get_var("ROADMAP_FILE_PATH").unwrap();
    let mut roadmap = configuration::load_roadmap(roadmap_file_path);

    for thematic in &mut roadmap {
        client.generate_knowledge(thematic).await?;
    }
    Ok(roadmap)
}

pub async fn evaluate_answer(answer: String, topic: Topic) -> Result<Answer, TirError> {
    let client = get_client();
    client.evaluate_answer(answer, topic).await
}

pub async fn correct_explanation(correction: String, topic: &mut Topic) -> Result<(), TirError> {
    let client = get_client();
    client.correct_explanation(correction, topic).await
}
