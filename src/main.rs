use rand::Rng;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct ConfigData {
    questions_filename: String,
    welcome_msg: String,
    correct: String,
    wrong: String,
}

fn read_config(config_file_content: String) -> ConfigData {
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

fn main() {
    const VER: &str = "STE v0.1.1a";

    let mut config = File::open("config").expect("Can't find a config file!");

    let mut config_content = String::new();

    config
        .read_to_string(&mut config_content)
        .expect("Can't read the config file.");

    let configuration = read_config(config_content);

    let mut test_file = File::open(configuration.questions_filename)
        .expect("Can't open the test file. Check config file and/or your test file.");

    let mut test_content: String = String::new();
    test_file
        .read_to_string(&mut test_content)
        .expect("Can't read the test file.");

    //TODO: change split character
    let questions: Vec<&str> = test_content.split("<q>").collect();

    println!("{}\n\n{: ^120}\n", VER, &configuration.welcome_msg);
    //TODO: Quick tutorial

    let mut q_lines: Vec<&str>;
    let mut q_number;
    let mut a_number: usize;
    let mut used_indices: Vec<usize> = [].to_vec();

    loop {
        q_number = rand::thread_rng().gen_range(0, questions.len());
        q_lines = questions[q_number].lines().collect();

        //Question
        println!("{}", &q_lines[1]);

        used_indices.clear();
        let mut index = 1;
        //Answers
        for i in (2..q_lines.len()) {
            loop {
                a_number = rand::thread_rng().gen_range(2, q_lines.len());

                if !used_indices.contains(&a_number) {
                    used_indices.push(a_number);
                    println!("[{}] {}", index, q_lines[a_number]);
                    break;
                }
            }
            index = index + 1;
        }

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read answer");

        let answer_idx = match answer.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("Can't parse.\n");
                256
            }
        };

        if answer_idx == 256 || answer_idx - 1 >= used_indices.len() {
            println!("Answer not available.\n");
            continue;
        }

        if used_indices[answer_idx - 1] == 2 {
            println!("{}\n", configuration.correct);
        } else {
            println!("{}\n", configuration.wrong);
        }
    }
}
