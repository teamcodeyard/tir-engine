use crate::structs::Thematic;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Read;

pub fn load_env(filename: String) {
    let _ = dotenv::from_filename(filename);
    dotenv().ok();
}

pub fn get_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

pub fn load_roadmap(roadmap_path: String) -> Vec<Thematic> {
    let mut file = File::open(roadmap_path).expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read config file");
    serde_yaml::from_str(&contents).expect("Failed to parse YAML")
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_env() {
        load_env(String::from("default.env"));
        let roadmap_file_path = get_var("ROADMAP_FILE_PATH");
        assert!(roadmap_file_path.is_some(), "ROADMAP_FILE_PATH doesn't exist");
    }

    #[test]
    fn test_get_var() {
        load_env(String::from("default.env"));
        let roadmap_file_path = get_var("ROADMAP_FILE_PATH");
        assert_eq!(roadmap_file_path.unwrap(), "./default.roadmap.yml");
    }
    
    #[test]
    fn test_load_env_negative() {
        load_env(String::from("default.env"));
        let roadmap_file_path = get_var("roadmap_file_path");
        assert!(roadmap_file_path.is_none(), "roadmap_file_path does exist");
    }

    #[test]
    fn test_load_roadmap() {
        let _ = load_roadmap(String::from("./default.roadmap.yml"));
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn test_load_roadmap_negative() {
        let _ = load_roadmap(String::from("./not.exists.yml"));
    }

}
