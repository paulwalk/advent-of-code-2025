use crate::day_8::model::{Circuits};
use crate::utilities::read_lines;
use std::cmp::{Ordering, PartialEq};
use crate::models::coords_3d::{compare_distance, Coord3D, Coord3DPair};

pub(crate) const DAY_NUM: u8 = 8;

pub fn solve_pt_1(data_file_path: &str, connections: u16) -> u64 {
    let pairs = get_sorted_pairs(data_file_path);
    let mut circuits = Circuits::new();
    for i in 0..connections {
        circuits.add_pair(pairs[i as usize].clone());
    }
    let mut sizes: Vec<usize> = vec![];
    for circuit in circuits.circuits.values() {
        sizes.push(circuit.junction_boxes.len());
    }
    sizes.sort();
    sizes.reverse();
    let mut running_total: u64 = 1;
    for i in 0..3 {
        running_total *= sizes[i] as u64;
    }
    running_total
}

pub fn solve_pt_2(data_file_path: &str) -> u64 {
    let pairs = get_sorted_pairs(data_file_path);
    let mut circuits = Circuits::new();
    let mut result_pair: Option<Coord3DPair> = None;
    for i in 0..pairs.len() {
        let number_of_jbs_added_to_a_circuit = circuits.add_pair(pairs[i].clone());
        if circuits.circuits.len() == 1 {
            if number_of_jbs_added_to_a_circuit > 0 {
                result_pair = Some(pairs[i].clone());
            }
        } else {
            log::debug!("Number of circuits: {}", circuits.circuits.len());
        }
    }
    let pair = result_pair.unwrap();
    (pair.p.x * pair.q.x) as u64
}

fn get_sorted_pairs(data_file_path: &str) -> Vec<Coord3DPair> {
    let mut points: Vec<Coord3D> = Vec::new();
    if let Ok(lines) = read_lines(data_file_path) {
        for line in lines.map_while(Result::ok) {
            let coords: Vec<i64> = line.split(',').map(|s| s.trim().parse::<i64>().unwrap()).collect();
            points.push(Coord3D {x:coords[0], y:coords[1], z:coords[2]});
        }
    }
    let mut pairs: Vec<Coord3DPair> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let pair = Coord3DPair{p:points[i].clone(), q:points[j].clone()};
            pairs.push(pair);
        }
    }
    pairs.sort_by(compare_distance);
    pairs
}


