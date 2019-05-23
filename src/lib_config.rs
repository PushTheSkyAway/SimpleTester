
#[derive(Debug)]
pub struct ConfigData {
    pub questions_filename: String,
    pub welcome_msg: String,
    pub correct: String,
    pub wrong: String,
}

pub fn read_config(config_file_content: String) -> ConfigData {
    // creating new ConfigData
    let mut cfg = ConfigData {
        questions_filename: String::new(),
        welcome_msg: String::new(),
        correct: String::new(),
        wrong: String::new(),
    };

    // splitting config file to lines
    let lines: Vec<&str> = config_file_content.lines().collect();

    // parsing every line
    for line in &lines {
        let param: Vec<&str> = line.split("::").collect();

        match param[0] {
            "questions_file" => {
                cfg.questions_filename = param[1].to_string();
            }
            "welcome_msg" => {
                cfg.welcome_msg = param[1].to_string();
            }
            "correct_answer" => {
                cfg.correct = param[1].to_string();
            }
            "wrong_answer" => {
                cfg.wrong = param[1].to_string();
            }
            _ => {
                println!("Unknown parameter in the config file!");
            }
        }
    }

    cfg
}
