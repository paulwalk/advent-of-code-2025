use crate::utilities::read_lines;

pub(crate) const DAY_NUM: u8 = 4;

const ROLL_OF_PAPER:char = '@';
const WILL_BE_REMOVED:char = '#';
const REMOVED:char = '.';

pub fn solve_pt_1(data_file_path: &str) -> u64 {
    let mut running_total: u64 = 0;
    if let Ok(lines) = read_lines(data_file_path) {
        let mut rows: Vec<Vec<char>> = lines.map_while(Result::ok).map(|line| line.chars().collect()).collect();
        running_total += process_grid(&mut rows);
    }
    running_total
}

pub fn solve_pt_2(data_file_path: &str) -> u64 {
    let mut running_total: u64 = 0;
    if let Ok(lines) = read_lines(data_file_path) {
        let mut rows: Vec<Vec<char>> = lines.map_while(Result::ok).map(|line| line.chars().collect()).collect();
        let mut subtotal = 1;
        while subtotal > 0 {
            subtotal = process_grid(&mut rows);
            running_total += subtotal;
        }
        // running_total += process_grid(&mut rows);
    }
    running_total
}

fn process_grid(rows: &mut Vec<Vec<char>>)  -> u64 {
    let mut running_total: u64 = 0;
    for row_index in 0..rows.len() {
        for col_index in 0..rows[row_index].len() {
            if rows[row_index][col_index] == ROLL_OF_PAPER {
                let mut neighbour_count = 0;
                if row_index > 0 {
                    neighbour_count += check_neighbour(rows, row_index - 1, col_index);
                    if col_index > 0 {
                        neighbour_count += check_neighbour(rows, row_index - 1, col_index - 1);
                    }
                    if col_index < rows[row_index].len() - 1 {
                        neighbour_count += check_neighbour(rows, row_index - 1, col_index + 1);
                    }
                }
                if row_index < rows.len() - 1 {
                    neighbour_count += check_neighbour(rows, row_index + 1, col_index);
                    if col_index > 0 {
                        neighbour_count += check_neighbour(rows, row_index + 1, col_index - 1);
                    }
                    if col_index < rows[row_index].len() - 1 {
                        neighbour_count += check_neighbour(rows, row_index + 1, col_index + 1);
                    }
                }
                if col_index > 0 {
                    neighbour_count += check_neighbour(rows, row_index, col_index - 1);
                }
                if col_index < rows[row_index].len() - 1 {
                    neighbour_count += check_neighbour(rows, row_index, col_index + 1);
                }
                if neighbour_count < 4 {
                    running_total += 1;
                    rows[row_index][col_index] = WILL_BE_REMOVED;
                }
            }
        }
    }
    for row_index in 0..rows.len() {
        for col_index in 0..rows[row_index].len() {
            if rows[row_index][col_index] == WILL_BE_REMOVED {
                rows[row_index][col_index] = REMOVED;
            }
        }
    }
    // log::debug!("rows: {:?}", rows);
    running_total
}

fn check_neighbour(rows: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> u8 {
    if rows[row_index][col_index] == ROLL_OF_PAPER || rows[row_index][col_index] == WILL_BE_REMOVED{ 1 } else { 0 }
}

