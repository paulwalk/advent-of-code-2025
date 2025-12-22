use crate::utilities::read_lines;

pub(crate) const DAY_NUM: u8 = 5;

pub fn solve_pt_1(data_file_path: &str) -> u64 {
    let mut running_total: u64 = 0;
    let ranges: Vec<(u64, u64)> = get_ranges(data_file_path);
    let ingredients: Vec<usize> = get_ingredients(data_file_path);
    for ingredient in ingredients {
        running_total += is_ingredient_in_any_range(ingredient, ranges.clone()) as u64;
    }
    running_total
}

pub fn solve_pt_2(data_file_path: &str) -> u64 {
    let mut running_total = 0;
    let mut input_ranges: Vec<(u64, u64)> = get_ranges(data_file_path);
    input_ranges.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut each_range_start = input_ranges[0].0;
    let mut each_range_end = input_ranges[0].1;
    for i in 0..input_ranges.len() - 1 {
        if input_ranges[i + 1].0 > input_ranges[i].1 {
            let sub_total = (each_range_end - each_range_start) + 1;
            running_total += sub_total;
            each_range_start = input_ranges[i + 1].0;
        }
        if input_ranges[i + 1].1 > each_range_end {
            each_range_end = input_ranges[i + 1].1;
        }

    }
    let sub_total = (each_range_end - each_range_start) + 1;
    running_total += sub_total;
    running_total
}

fn get_ranges(data_file_path: &str) -> Vec<(u64, u64)> {
    let mut ranges: Vec<(u64, u64)> = vec![];
    if let Ok(lines) = read_lines(data_file_path) {
        for line in lines.map_while(Result::ok) {
            if line.contains("-") {
                let range_pair = line.split_once("-").unwrap();
                let start = range_pair.0.parse::<u64>().unwrap();
                let end = range_pair.1.parse::<u64>().unwrap();
                ranges.push((start, end));
            } else {
                break;
            }
        }
    }
    ranges
}

fn get_ingredients(data_file_path: &str) -> Vec<usize> {
    let mut ingredients: Vec<usize> = vec![];
    if let Ok(lines) = read_lines(data_file_path) {
        for line in lines.map_while(Result::ok) {
            if line.contains("-") {
                // is a range line
            } else if line.is_empty() {
                // is a blanks line separating sections
            } else {
                ingredients.push(line.parse::<usize>().unwrap());
            }
        }
    }
    ingredients
}

pub fn is_ingredient_in_any_range(ingredient: usize, ranges: Vec<(u64, u64)>) -> u8 {
    for range in &ranges {
        if range.0 <= ingredient as u64 && ingredient as u64 <= range.1 {
            return 1;
        }
    }
    0
}

