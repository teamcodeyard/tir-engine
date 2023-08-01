mod configuration;
mod openai;
mod structs;
use structs::{AnswerResult, Thematic, Topic};

fn get_client() -> openai::GPT {
    configuration::load_env();
    let secret_key = configuration::get_var("OPENAI_SK").unwrap();
    openai::GPT::new(secret_key)
}

pub async fn generate_knowledge() -> Vec<Thematic> {
    let client = get_client();
    let mut roadmap = configuration::load_roadmap();
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

#[cfg(test)]
mod tests {

    use crate::configuration::{self};
    use crate::openai::{self};
    use crate::structs::{Thematic, Topic};
    use std::vec;
    use tokio::test;

    #[test]
    async fn generate_knowledge() {
        configuration::load_env();
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
        let client = openai::GPT::new(secret_key);
        let mut thematic = Thematic {
            title: String::from("Desing patterns"),
            topics: vec![Topic {
                title: String::from("Singleton"),
                explanation: Some(String::from("")),
            }],
        };

        client.generate_knowledge(&mut thematic).await;
        println!("\x1b[1;36m{:?}\x1b[1;0m", thematic);
    }

    #[test]
    async fn evaluate_answer() {
        configuration::load_env();
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
        let client = openai::GPT::new(secret_key);

        let topic = Topic {
            title: String::from("Singleton design pattern"),
            explanation: Some(String::from("")),
        };

        let result = client.evaluate_answer(String::from("Singleton is a design pattern in the software development when you only have one instance from a Class."), topic).await;
        println!("\x1b[1;36m{:?}\x1b[1;0m", result);
    }

    #[test]
    async fn correct_explanation() {
        configuration::load_env();
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
        let client = openai::GPT::new(secret_key);

        let mut topic = Topic {
            title: String::from("Singleton design pattern"),
            explanation: Some(String::from("The Singleton pattern is a design pattern that ensures only one instance of a class is created throughout the application. It is useful when you want to restrict the instantiation of a class to a single object and ensure that no other object can create additional instances.")),
        };

        client
            .correct_explanation(
                String::from(
                    "Please extend your answer with global access point to the signleton instance",
                ),
                &mut topic,
            )
            .await;
        println!("\x1b[1;36m{:?}\x1b[1;0m", topic);
    }
}
