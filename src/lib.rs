mod configuration;
mod openai;

fn get_client() -> openai::GPT {
    configuration::load_env();
    let secret_key = configuration::get_var("OPENAI_SK").unwrap();
    openai::GPT::new(secret_key)
}

pub fn generate_knowledge() {

    // load env
    // call open ai
    // return results
}

pub fn generate_lessons(/* knowledge */) {
    // load env
    // call open ai
    // return results
}

pub fn evaluate_lesson(/* lesson */) {
    // load env
    // call open ai
    // return results
}

#[cfg(test)]
mod tests {

    use crate::{
        configuration::{self, Thematic},
        openai,
    };

    use tokio::test;

    #[test]
    async fn it_works() {
        configuration::load_env();
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
        let client = openai::GPT::new(secret_key);
        let roadmap = configuration::load_roadmap();
        let thematic = roadmap.first().unwrap().clone();
        /* Ugly solution, need to fix references. Don't judge me, please! */
        let mut owned_thematic = Thematic {
            title: thematic.title.clone(),
            topics: thematic.topics.to_vec(),
        };

        client.generate_knowledge(&mut owned_thematic).await;
        println!("\x1b[1;36m{:?}\x1b[1;0m", owned_thematic);
    }
}
