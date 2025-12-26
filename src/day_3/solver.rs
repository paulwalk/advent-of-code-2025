use crate::lib_common::utilities::read_lines;

pub(crate) const DAY_NUM: u8 = 3;

pub fn solve_pt_1(data_file_path: &str) -> u64{
    let mut running_total: u64 = 0;
    if let Ok(lines) = read_lines(data_file_path) {
        for line in lines.map_while(Result::ok) {
            let mut first_max: u8 = 0;
            let mut first_max_index = 0;
            for i in 0..=line.len() - 2 {
                let digit = line[i..=i].parse::<u8>().unwrap();
                if digit > first_max {
                    first_max = digit;
                    first_max_index = i;
                }
            }
            let mut second_max: u8 = 0;
            for i in first_max_index + 1..=line.len() - 1 {
                let digit = line[i..=i].parse::<u8>().unwrap();
                if digit > second_max {
                    second_max = digit;
                }
            }
            let number_pair_as_sting = format!("{}{}", first_max, second_max);
            let number_pair = number_pair_as_sting.parse::<u8>().unwrap();
            running_total += number_pair as u64;
        }
    }
    running_total
}

pub fn solve_pt_2(data_file_path: &str) -> u64{
    let mut running_total: u64 = 0;
    if let Ok(lines) = read_lines(data_file_path) {
        for line in lines.map_while(Result::ok) {
            let mut sliding_window_size: usize = line.len() - 12 ;
            let mut target: String = "".to_string();
            let mut candidates = line.clone();
            while target.len() < 12 {
                let number_and_index = get_highest_number_and_index(&candidates, sliding_window_size );
                target.push(number_and_index.0.to_string().chars().next().unwrap());
                candidates = candidates[(number_and_index.1+1)..].to_string();
                let remaining_slots = 12 - target.len();
                sliding_window_size = (candidates.len() - remaining_slots) + 1;
            }
            let target_number = target.parse::<u64>().unwrap();
            running_total += target_number;
        }
    }
    running_total
}

fn get_highest_number_and_index(candidates: &str, sliding_window_size: usize) -> (u8, usize) {
    let end = sliding_window_size;
    let _window = &candidates[0..end];
    let mut highest_number: u8 = candidates[0..=0].parse::<u8>().unwrap();
    let mut highest_index: usize = 0;
    for i in 0..end {
        let digit = candidates[i..=i].parse::<u8>().expect("digit not parseable");
        if digit > highest_number {
            highest_number = digit;
            highest_index = i;
        }
    }
    (highest_number, highest_index)
}

