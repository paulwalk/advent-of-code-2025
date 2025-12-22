use crate::models::coord_vector::MinMax;
use crate::models::coords_2d::{compare_area, Coord2D, Coord2DPair};
use crate::utilities::read_lines;
use std::collections::HashSet;

pub(crate) const DAY_NUM: u8 = 9;

pub fn solve_pt_1(data_file_path: &str) -> u64 {
    let mut coords: Vec<(Coord2D)> = vec![];
    let mut sorted_red_pairs: Vec<Coord2DPair> = vec![];
    (coords, sorted_red_pairs) = new_from_data_file(data_file_path);
    let pair = sorted_red_pairs.first().unwrap();
    // print(coords);
    pair.area
}

pub fn solve_pt_2(data_file_path: &str) -> u64 {
    let mut area: u64 = 0;
    let mut coords: Vec<(Coord2D)> = vec![];
    let mut sorted_red_pairs: Vec<Coord2DPair> = vec![];
    (coords, sorted_red_pairs) = new_from_data_file(data_file_path);
    fill_in_green_squares(&mut coords);
    let coords_hash_set: HashSet<Coord2D> = HashSet::from_iter(coords.iter().cloned());
    for pair in sorted_red_pairs {
        let coords_in_rectangle: HashSet<Coord2D> = set_of_coords_in_rectangle(pair.p.clone(), pair.q.clone());
        if coords_in_rectangle.difference(&coords_hash_set).count() == 0 {
            area = pair.area;
            break;
        }
    }
    area
}

pub fn new_from_data_file(data_file_path: &str) -> (Vec<(Coord2D)>, Vec<Coord2DPair>) {
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

pub fn print(coords: Vec<(Coord2D)>) {
    let grid_row = vec!['.'; (coords.max_x() + 1) as usize];
    let mut grid: Vec<Vec<char>> = vec![];
    for _ in 0..(coords.max_y() + 1) {
        grid.push(grid_row.clone());
    }
    for point in coords.clone() {
        grid[(point.y) as usize][(point.x) as usize] = '@';
    }
    for row in grid {
        let row_string: String = row.iter().map(|x| x.to_string()).collect();
        print!("{}\n", row_string);
    }
}

pub fn fill_in_green_squares(mut coords: &mut Vec<(Coord2D)>) {
    let first_and_last_coords = (coords[0].clone(), coords[coords.len() - 1].clone());
    for i in 0..coords.len() - 1 {
        fill_in_green_squares_for_coords(coords, coords[i].clone(), coords[i + 1].clone());
    }
    fill_in_green_squares_for_coords(coords, first_and_last_coords.0, first_and_last_coords.1);
    log::debug!("Filled in green squares, total coords now: {}", coords.len());
    log::debug!("Min y: {}, Max y: {}", coords.min_y(), coords.max_y());
    for row_index in coords.min_y()..=coords.max_y() {
        let min_x = coords.iter().filter(|p| p.y == row_index).map(|p| p.x).min().expect("Could not find min");
        let max_x = coords.iter().filter(|p| p.y == row_index).map(|p| p.x).max().expect("Could not find max");
        for x in min_x + 1..max_x {
            if !coords.contains(&Coord2D { x, y: row_index }) {
                coords.push(Coord2D { x, y: row_index });
            }
        }
    }
}

pub fn fill_in_green_squares_for_coords(mut coords: &mut Vec<(Coord2D)>, point_1: Coord2D, point_2: Coord2D) {
    if point_1.x == point_2.x {
        if point_1.y < point_2.y {
            for j in point_1.y + 1..point_2.y {
                coords.push((Coord2D { x: point_1.x, y: j }));
            }
        } else if point_1.y > point_2.y {
            for j in point_2.y + 1..point_1.y {
                coords.push((Coord2D { x: point_1.x, y: j }));
            }
        }
    } else if point_1.y == point_2.y {
        if point_1.x < point_2.x {
            for j in point_1.x + 1..point_2.x {
                coords.push((Coord2D { x: j, y: point_1.y }));
            }
        } else if point_1.x > point_2.x {
            for j in point_2.x + 1..point_1.x {
                coords.push((Coord2D { x: j, y: point_1.y }));
            }
        }
    } else {
        panic!("Coordinates do not form a chain");
    }
    // coords
}

pub fn set_of_coords_in_rectangle(p: Coord2D, q: Coord2D) -> HashSet<Coord2D> {
    let mut coords_in_rectangle: HashSet<Coord2D> = HashSet::new();
    let min_x: i64;
    let max_x: i64;
    let min_y: i64;
    let max_y: i64;
    if p.x <= q.x {
        min_x = p.x;
        max_x = q.x;
    } else {
        min_x = q.x;
        max_x = p.x;
    }
    if p.y <= q.y {
        min_y = p.y;
        max_y = q.y;
    } else {
        min_y = q.y;
        max_y = p.y;
    }
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            coords_in_rectangle.insert(Coord2D { x, y });
        }
    }
    coords_in_rectangle
}
