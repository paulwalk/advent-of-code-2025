use crate::lib_common::coords_2d::{compare_area, Coord2D, Coord2DPair};
use crate::lib_common::utilities::read_lines;
use geo::{point, Contains, LineString, Polygon};

pub(crate) const DAY_NUM: u8 = 9;

pub fn solve_pt_1(data_file_path: &str) -> u64 {
    let coords: Vec<Coord2D>;
    let sorted_red_pairs: Vec<Coord2DPair>;
    (coords, sorted_red_pairs) = new_from_data_file(data_file_path);
    let pair = sorted_red_pairs.first().unwrap();
    pair.area
}

pub fn solve_pt_2(data_file_path: &str) -> u64 {
    let mut area: u64 = 0;
    let mut coords: Vec<Coord2D > = vec![];
    let mut sorted_red_pairs: Vec<Coord2DPair> = vec![];
    (coords, sorted_red_pairs) = new_from_data_file(data_file_path);
    let mut raw_coord_vector:Vec<(f64,f64)> = vec![];
    for coord in coords.clone() {
        raw_coord_vector.push((coord.x as f64, coord.y as f64));
    }
    let polygon = Polygon::new(
        LineString::from(raw_coord_vector),
        vec![],
    );
    
    for red_pair in sorted_red_pairs {
        if polygon.contains(&red_pair.geo_rectangle()) {
            area = red_pair.area;
            break;
        }
    }
    area
}

pub fn new_from_data_file(data_file_path: &str) -> (Vec<Coord2D >, Vec<Coord2DPair>) {
    let mut red_coords: Vec<Coord2D> = vec![];
    if let Ok(lines) = read_lines(data_file_path) {
        for line in lines.map_while(Result::ok) {
            let string_pair = line.split_once(",").expect("Could not split coordinate pair");
            let x: i64 = string_pair.0.parse::<i64>().expect("Could not parse x coordinate");
            let y: i64 = string_pair.1.parse::<i64>().expect("Could not parse y coordinate");
            red_coords.push(Coord2D { x, y });
        }
    }
    let mut sorted_red_pairs: Vec<Coord2DPair> = Vec::new();
    for i in 0..red_coords.len() {
        for j in i + 1..red_coords.len() {
            let pair = Coord2DPair::new(red_coords[i].clone(), red_coords[j].clone());
            sorted_red_pairs.push(pair);
        }
    }
    sorted_red_pairs.sort_by(compare_area);
    sorted_red_pairs.reverse();
    (red_coords, sorted_red_pairs)
}
