use crate::day_10::models::{buttons_to_lights, new_from_data_file, Machine};
use itertools::Itertools;

pub(crate) const DAY_NUM: u8 = 10;

pub fn solve_pt_1(data_file_path: &str) -> u64 {
    let mut running_total: u64 = 0;
    let machines: Vec<Machine> = new_from_data_file(data_file_path);
    for (machine_index,machine) in machines.iter().enumerate() {
        let mut match_found: bool = false;
        for button in machine.buttons.clone() {
            if button == machine.lights {
                running_total += 1;
                match_found = true;
                break;
            }
        }
        let mut i: u64 = 2;
        while !match_found && i <= machine.buttons.clone().len() as u64 {
            for combo in machine.buttons.clone().into_iter().combinations(i as usize) {
                if buttons_to_lights(combo.clone()) == machine.lights {
                    running_total += i;
                    match_found = true;
                    break;
                }
            }
            i += 1;
        }
        if !match_found {
            log::debug!("Iterations: {}", i-1);
            log::debug!("No button combination found for machine {}: {:?}", machine_index,machine);
            return 0;
        } else {
            log::debug!("FOUND! Iterations: {}", i-1);
        }
    }
    running_total
}

pub fn solve_pt_2(data_file_path: &str) -> u64 {
    0
}
