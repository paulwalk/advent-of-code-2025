use crate::day_10::lights::{light_vector_from_chars};
use crate::day_10::lights::Light;
use crate::utilities::read_lines;

pub(crate) const DAY_NUM: u8 = 10;


pub fn solve_pt_1(data_file_path: &str) -> u64 {
    let lights = new_from_data_file(data_file_path);
    log::debug!("Lights: {:?}", lights);
    0
}

pub fn solve_pt_2(data_file_path: &str) -> u64 {
    0
}

pub fn new_from_data_file(data_file_path: &str) -> Vec<Vec<Light>> {
    let mut lights: Vec<Vec<Light>> = vec![];
    if let Ok(lines) = read_lines(data_file_path) {
        for line in lines.map_while(Result::ok) {
            for section in line.split(" ") {
                let first_char = section.chars().next().unwrap();
                match first_char {
                    '[' => {
                        lights.push(light_vector_from_chars(section[1..section.len()-1].chars().collect()));
                    }
                    ,
                    '(' => {},
                    '{' => {},
                    _ => panic!("Invalid character: {}", first_char),
                }
            }
        }
    }
    lights
}

