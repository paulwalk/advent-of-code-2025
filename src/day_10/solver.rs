use crate::day_10::models::light_vector_from_chars;
use crate::day_10::models::Light;
use crate::utilities::read_lines;

pub(crate) const DAY_NUM: u8 = 10;

pub fn solve_pt_1(data_file_path: &str) -> u64 {
    let (lights, buttons,_jolts) = new_from_data_file(data_file_path);
    log::debug!("Lights: {:?}", lights);
    log::debug!("Buttons: {:?}", buttons);
    0
}

pub fn solve_pt_2(data_file_path: &str) -> u64 {
    0
}

pub fn new_from_data_file(data_file_path: &str) -> (Vec<Vec<Light>>, Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut lights: Vec<Vec<Light>> = vec![];
    let mut buttons: Vec<Vec<u8>> = vec![];
    let mut jolts: Vec<Vec<u8>> = vec![];
    if let Ok(lines) = read_lines(data_file_path) {
        for line in lines.map_while(Result::ok) {
            for section in line.split(" ") {
                let first_char = section.chars().next().unwrap();
                match first_char {
                    '[' => {
                        lights.push(light_vector_from_chars(section[1..section.len() - 1].chars().collect()));
                    }
                    '(' => {
                        buttons.push(section[1..section.len() - 1].split(',').map(|s| s.parse::<u8>().unwrap()).collect());
                    }
                    '{' => {
                        jolts.push(section[1..section.len() - 1].split(',').map(|s| s.parse::<u8>().unwrap()).collect());
                    }
                    _ => panic!("Invalid character: {}", first_char),
                }
            }
        }
    }
    (lights, buttons, jolts)
}
