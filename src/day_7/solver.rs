use crate::utilities::read_lines;

pub(crate) const DAY_NUM: u8 = 7;

const BEAM: char = '|';
const START: char = 'S';
const EMPTY: char = '.';
const SPLITTER: char = '^';

pub fn solve_pt_1(data_file_path: &str) -> u64 {
    let mut running_total: u64 = 0;
    let mut grid = create_grid(data_file_path);
    // dump_grid(&grid);
    grid = get_full_tree(grid);
    for row in 1..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row - 1][col] == BEAM && grid[row][col] == SPLITTER {
                running_total += 1;
            }
        }
    }
    // dump_grid(&grid);
    running_total
}

pub fn solve_pt_2(data_file_path: &str) -> u64 {
    let mut running_total: u64 = 0;
    let mut grid = create_grid(data_file_path);
    let mut value_grid: Vec<Vec<u64>> = vec![];
    value_grid.push(vec![0; grid[0].len()]);
    grid = get_full_tree(grid);
    for row in 1..grid.len() {
        let mut value_row: Vec<u64> = vec![];
        for col in 0..grid[0].len() {
            if grid[row][col] == BEAM && grid[row - 1][col] == START {
                value_row.push(1);
            } else if grid[row][col] == BEAM {
                let mut subtotal: u64 = 0;
                if grid[row - 1][col] == BEAM {
                    subtotal += value_grid[row - 1][col];
                }
                if col > 0 && grid[row - 1][col - 1] == BEAM && grid[row][col - 1] == SPLITTER {
                    subtotal += value_grid[row - 1][col - 1];
                }
                if col < value_grid[0].len() - 1 && grid[row - 1][col + 1] == BEAM && grid[row][col + 1] == SPLITTER {
                    subtotal += value_grid[row - 1][col + 1];
                }
                value_row.push(subtotal);
            } else {
                value_row.push(0);
            }
        }
        value_grid.push(value_row);
    }
    let final_row_of_values = value_grid.last().unwrap();
    for col in final_row_of_values {
        running_total += col;
    }
    // dump_grid(&value_grid);
    running_total
}

pub fn create_grid(data_file_path: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];
    if let Ok(lines) = read_lines(data_file_path) {
        for line in lines.map_while(Result::ok) {
            let row: Vec<char> = line.chars().collect();
            grid.push(row.clone());
        }
    }
    grid
}

pub fn dump_grid<T>(grid: &Vec<Vec<T>>)
where
    T: std::fmt::Display,
{
    for row in grid {
        for col in row {
            print!("{:>5}", col);
        }
        println!();
    }
    println!();
}

fn get_full_tree(input_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid = input_grid.clone();
    for row in 1..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row - 1][col] == START {
                grid[row][col] = BEAM;
            } else if grid[row - 1][col] == BEAM && grid[row][col] == EMPTY {
                grid[row][col] = BEAM;
            } else if grid[row - 1][col] == BEAM && grid[row][col] == SPLITTER {
                grid[row][col - 1] = BEAM;
                grid[row][col + 1] = BEAM;
            }
        }
    }
    grid
}

