
pub struct Day {
    pub num: u8,
    pub data_path: String,
    pub test_data_path: String,
}

impl Day {
    pub fn new(num: u8, puzzle_dir: String) -> Day {
        Day {
            num,
            data_path: format!("{}/{}/data.txt", puzzle_dir, num),
            test_data_path: format!("{}/{}/test_data.txt", puzzle_dir, num),
        }
    }

    pub fn label(&self) -> String {
        format!("Day {}", self.num)
    }
}
