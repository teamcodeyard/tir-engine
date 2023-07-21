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

    use crate::{configuration, openai};
    
    use tokio::test;
    
    #[test]
    async fn it_works() {
        configuration::load_env();
        let secret_key = configuration::get_var("OPENAI_SK").unwrap();
        let client = openai::GPT::new(secret_key);
        let roadmap = configuration::load_roadmap();
        let completion = client.generate_knowledge(roadmap).await;
        let choice = completion.choices.first().unwrap();
        println!("\x1b[1;36m{}\x1b[1;0m", choice.message.content);
        assert_eq!(completion.choices.len(), 1);
    }
}
