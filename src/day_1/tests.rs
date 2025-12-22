#[cfg(test)]
mod tests {
    use crate::config::AppConfiguration;
    use crate::day_1::solver::{solve, DAY_NUM};
    use crate::day::Day;

    #[test]
    fn test_solve_day_1_test_data() {
        let app_config = &AppConfiguration::new();
        let day: Day = Day::new(DAY_NUM, app_config.puzzle_dir.clone());
        assert_eq!(solve(day.test_data_path.as_str()), 3);
    }

    #[test]
    fn test_solve_day_1() {
        let app_config = &AppConfiguration::new();
        let day: Day = Day::new(DAY_NUM, app_config.puzzle_dir.clone());
        assert_eq!(solve(day.data_path.as_str()), app_config.answers[&DAY_NUM][0]);
    }
}