use std::collections::HashMap;
use std::env;
use dotenvy::dotenv;



#[derive(Debug,Clone)]
pub struct AppConfiguration {
    pub log_spec: String,
    pub puzzle_dir: String,
    pub answers: HashMap<u8,Vec<u64>>,
}

impl AppConfiguration {
    pub fn new() -> AppConfiguration {
        dotenv().ok();
        let puzzle_dir = env::var("PUZZLE_DIR").expect("PUZZLE_DIR must be set");
        let log_spec = env::var("LOG_SPEC").unwrap_or("info".to_string());
        let mut answers:HashMap<u8,Vec<u64>> = HashMap::new();
        for day_num in 1..=12u8 {
            let answers_string_line = env::var(format!("DAY_{}", day_num)).unwrap_or("".to_string());
            let answers_string_pair:Vec<&str> = answers_string_line.split(',').collect();
            let mut answers_for_day:Vec<u64> = vec![];
            for answer_string in &answers_string_pair {
                answers_for_day.push(answer_string.trim().parse::<u64>().unwrap_or(0));
            }
            answers.insert(day_num, answers_for_day);
        }
        
        AppConfiguration {
            puzzle_dir,
            log_spec,
            answers,
        }
    }
}

