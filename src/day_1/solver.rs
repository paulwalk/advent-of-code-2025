use crate::lib_common::utilities::read_lines;

pub(crate) const DAY_NUM: u8 = 1;

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug)]
pub struct Turn {
    direction: Direction,
    distance: i16,
}

impl Turn {
    pub fn new(clue: String) -> Self {
        let direction = match &clue[0..1] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!("Invalid direction"),
        };
        let distance = &clue[1..].parse::<i16>().expect("Could not parse distance as i16");
        Turn { direction, distance: *distance }
    }
}



pub fn solve(data_file_path: &str) -> u64 {
    let mut zero_count:u64 = 0;
    if let Ok(lines) = read_lines(data_file_path) {

        let range_length = 100;
        let mut position = 50;
        for line in lines.map_while(Result::ok) {
            let turn = Turn::new(line);
            match turn.direction {
                Direction::Right => {
                    position = (position + range_length + turn.distance) % range_length;
                },
                Direction::Left => {
                    position = (position + range_length - turn.distance) % range_length;
                },
            }
            if position == 0 {
                zero_count += 1;
            }
        }
        log::info!("Number of times at zero position: {}", zero_count);
    }
    zero_count
}

