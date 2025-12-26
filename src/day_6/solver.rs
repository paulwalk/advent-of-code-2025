use crate::lib_common::utilities::read_lines;

pub(crate) const DAY_NUM: u8 = 6;

pub fn solve_pt_1(data_file_path: &str) -> i64 {
    let mut running_total = 0;
    let mut grid: Vec<Vec<String>> = vec![];
    if let Ok(lines) = read_lines(data_file_path) {
        for line in lines.map_while(Result::ok) {
            let row: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
            grid.push(row);
        }
    }
    for col in 0..grid[0].len() {
        let mut numbers: Vec<i64> = vec![];
        for row in 0..grid.len() - 1 {
            let value = &grid[row][col];
            if value.parse::<i64>().is_ok() {
                numbers.push(value.parse::<i64>().unwrap());
            } else {
                panic!("Invalid value in grid: {}", value);
            }
        }
        if grid[grid.len() - 1][col ] == "*" {
            let mut subtotal: i64 = 1;
            for value in numbers {
                subtotal *= value;
            }
            running_total += subtotal;
        } else if grid[grid.len() - 1][col ] == "+" {
            let mut subtotal: i64 = 0;
            for value in numbers {
                subtotal += value;
            }
            running_total += subtotal;
        } else {
            panic!("Invalid operation in grid: {}", grid[grid.len() - 1][col]);
        }
    }
    running_total
}

pub fn solve_pt_2(data_file_path: &str) -> i64 {
    let mut grid: Vec<Vec<char>> = vec![];
    if let Ok(lines) = read_lines(data_file_path) {
        for line in lines.map_while(Result::ok) {
            let row: Vec<char> = line.chars().collect();
            grid.push(row.clone());
            // log::debug!("Row: {:?}", row.clone().len());
        }
    }
    let mut separator_columns: Vec<u64> = vec![];
    for col_index in 0..grid[0].len() {
        let mut matched = true;
        for row_index in 0..grid.len() {
            if grid[row_index][col_index] != ' ' {
                matched = false;
                break;
            }
        }
        if matched {
            separator_columns.push(col_index as u64);
        }
    }
    // log::debug!("Separator cols: {:?}", separator_columns);
    let mut running_total:i64 = 0;
    let mut operator = get_operator_from_column(&grid, 0);
    let mut subtotal:i64 = 0;
    for col_index in 0..grid[0].len() {
        if separator_columns.contains(&(col_index as u64)) {
            running_total += subtotal;
            operator = None;
            subtotal = 0;
            continue;
        }
        if operator.is_none() {
            operator = get_operator_from_column(&grid, col_index as u64);
        }
        let number = get_number_from_column(&grid, col_index as u64);
        match operator {
            Some('+') => {
                subtotal += number as i64;
            }
            Some('*') => {
                if subtotal == 0 {
                    subtotal = 1;
                }
                subtotal *= number as i64;
            }
            None => {}
            _ => {}
        }
    }
    running_total += subtotal;
    running_total
}

pub fn get_operator_from_column(grid: &Vec<Vec<char>>, col_index: u64) -> Option<char> {
    if grid[grid.len() - 1][col_index as usize] == '+' {
        return Some('+');
    } else if grid[grid.len() - 1][col_index as usize] == '*' {
        return Some('*');
    }
    None
}

pub fn get_number_from_column(grid: &Vec<Vec<char>>, col_index: u64) -> u64 {
    let mut number_string = String::new();
    for row in 0..grid.len() {
        let value:String = grid[row][col_index as usize].to_string();
        if value.parse::<i64>().is_ok() {
            number_string.push_str(&value);
        }
    }
    if number_string.parse::<u64>().is_ok() {
        number_string.parse::<u64>().unwrap()
    } else {
        0
    }
}

