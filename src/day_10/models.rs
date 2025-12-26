use crate::lib_common::utilities::read_lines;

#[derive(Debug)]
pub struct Machine {
    pub lights: Vec<u8>,
    pub buttons: Vec<Vec<u8>>,
    pub jolts: Vec<u16>,
}

impl Machine {
    pub fn new(lights: Vec<u8>, buttons: Vec<Vec<u8>>, jolts: Vec<u16>) -> Self {
        Self { lights, buttons, jolts }
    }

    pub fn lights_panel_size(&self) -> u8 {
        self.lights.len() as u8
    }
}

pub fn parse_light_vector(chars: Vec<char>) -> Vec<u8> {
    chars
        .into_iter()
        .map(|c| match c {
            '#' => 1u8,
            '.' => 0u8,
            _ => panic!("Invalid character: {}", c),
        })
        .collect()
}

pub fn parse_button_vector(button_data: Vec<u8>, light_count: usize) -> Vec<u8> {
    let mut buttons: Vec<u8> = vec![0u8; light_count as usize];
    for button in button_data {
        // log::debug!("Button data: {:?}", button);
        buttons[button as usize] = 1u8;
    }
    buttons
}

pub fn new_from_data_file(data_file_path: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = vec![];
    if let Ok(lines) = read_lines(data_file_path) {
        for line in lines.map_while(Result::ok) {
            let mut buttons: Vec<Vec<u8>> = vec![];
            let mut lights: Vec<u8> = vec![];
            let mut jolts: Vec<u16> = vec![];
            for section in line.split(" ") {
                let first_char = section.chars().next().unwrap();
                match first_char {
                    '[' => {
                        lights = parse_light_vector(section[1..section.len() - 1].chars().collect());
                    }
                    '(' => {
                        buttons.push(parse_button_vector(section[1..section.len() - 1].split(',').map(|s| s.parse::<u8>().unwrap()).collect(), lights.len()));
                    }
                    '{' => {
                        jolts = section[1..section.len() - 1].split(',').map(|s| s.parse::<u16>().unwrap()).collect();
                    }
                    _ => panic!("Invalid character: {}", first_char),
                }
            }
            machines.push(Machine::new(lights, buttons, jolts));
        }
    }
    machines
}

pub fn buttons_to_lights(buttons: Vec<Vec<u8>>) -> Vec<u8> {
    let mut lights: Vec<u8> = vec![0u8; buttons[0].len()];
    for button in buttons {
        for (i, &b) in button.iter().enumerate() {
            lights[i] ^= b;
        }
    }
    lights
}