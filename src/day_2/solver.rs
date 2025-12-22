use std::fs;
use divisors_fixed::Divisors;

pub(crate) const DAY_NUM: u8 = 2;

pub fn solve_pt_1(data_file_path: &str) -> u64{
    let input: String = fs::read_to_string(data_file_path).expect("Failed to read data file");
    let mut running_total: u64 = 0;
    let ranges = input.split(",");
    for range in ranges {
        let boundaries = range.split("-");
        let pair: Vec<&str> = boundaries.collect();
        let lower = pair[0].parse::<u64>().expect("Could not parse clue as u64");
        let upper = pair[1].parse::<u64>().expect("Could not parse clue as u64");
        for number in lower..=upper {
            let number_string = number.to_string();
            if number_string.len().is_multiple_of(2) {
                let left = &number_string[0..number_string.len() / 2];
                let right = &number_string[number_string.len() / 2..number_string.len()];
                if left == right {
                    running_total += number;
                }
            }
        }
    }
    running_total
}

pub fn solve_pt_2(data_file_path: &str) -> u64{
    let input: String = fs::read_to_string(data_file_path).expect("Failed to read data file");
    let mut running_total: u64 = 0;
    let ranges = input.split(",");
    for range in ranges {
        let boundaries = range.split("-");
        let pair: Vec<&str> = boundaries.collect();
        let lower = pair[0].parse::<u64>().expect("Could not parse clue as u64");
        let upper = pair[1].parse::<u64>().expect("Could not parse clue as u64");
        for number in lower..=upper {
            if process_number_for_any_matching_sequence(number) {
                running_total += number;
            }
        }
    }
    running_total
}

fn process_number_for_any_matching_sequence(number: u64) -> bool {
    let number_string = number.to_string();
    let length = number_string.len();
    let mut divisors: Vec<usize> = length.divisors();
    divisors.remove(divisors.len() - 1);
    for divisor in divisors {
        let mut left_index = 0;
        let mut right_index = divisor;
        let segment_length = divisor-1;
        let mut matching = true;
        let max_iterations = (length/divisor)-1;
        let mut i = 1;
        while (i <= max_iterations) && matching {
            let left_value = &number_string[left_index..=left_index + segment_length];
            let right_value = &number_string[right_index..=right_index + segment_length];
            if left_value != right_value {
                matching = false;
            }
            left_index += divisor;
            right_index += divisor;
            i+=1;
        }
        if matching {
            // log::debug!("MATCH! Number = {} with divisor : {} ", number,divisor);
            return true;
        }
    }
    false
}

