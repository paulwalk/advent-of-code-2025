#[cfg(test)]
mod tests {
    use crate::config::AppConfiguration;
    use crate::day_6::solver::{solve_pt_1, solve_pt_2, DAY_NUM};
    use crate::day::Day;

    #[test]
    fn test_solve_part_1() {
        let app_config = &AppConfiguration::new();
        let day: Day = Day::new(DAY_NUM, app_config.puzzle_dir.clone());
        assert_eq!(solve_pt_1(day.test_data_path.as_str()), 4277556);
    }

    #[test]
    fn test_solve_part_2() {
        let app_config = &AppConfiguration::new();
        let day: Day = Day::new(DAY_NUM, app_config.puzzle_dir.clone());
        assert_eq!(solve_pt_2(day.test_data_path.as_str()), 3263827);
    }

    #[test]
    fn test_solve_day_6_part_1_test_data() {
        let app_config = &AppConfiguration::new();
        let day: Day = Day::new(DAY_NUM, app_config.puzzle_dir.clone());
        assert_eq!(solve_pt_1(day.data_path.as_str()), app_config.answers[&DAY_NUM][0] as i64);
    }

    #[test]
    fn test_solve_day_6_part_2_test_data() {
        let app_config = &AppConfiguration::new();
        let day: Day = Day::new(DAY_NUM, app_config.puzzle_dir.clone());
        assert_eq!(solve_pt_2(day.data_path.as_str()), app_config.answers[&DAY_NUM][1] as i64);
    }
}