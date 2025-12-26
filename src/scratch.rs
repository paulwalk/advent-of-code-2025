use itertools::Itertools;
use crate::day_10::models::buttons_to_lights;

pub fn scratch() {
    log::debug!("Starting scratch function");

    let lights: Vec<u8> = vec![0, 1, 1, 0];
    // let buttons:Vec<Vec<u8>> = [[0, 1, 1, 0, 0], [0, 1, 1, 1, 0], [0, 0, 0, 1, 1], [1, 1, 1, 0, 0], [0, 1, 0, 1, 1]].into_vec();
    // for combo in buttons.clone().into_iter().combinations(2 as usize) {
    //     if buttons_to_lights(combo.clone()) == lights {
    //         log::debug!("Found light combination: {:?}", combo.clone());
    //         // break;
    //     }
    // }

    log::info!("Scratch process Completed");
}

