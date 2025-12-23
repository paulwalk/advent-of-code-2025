use crate::cli::{Cli, Commands};
use crate::config::AppConfiguration;
use crate::day::Day;
use crate::logging::configure_flexi_logger;
use crate::scratch::scratch;
use clap::Parser;
mod cli;
mod config;
mod day;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod logging;
mod scratch;
mod utilities;
mod models;
mod day_10;

fn main() {
    let app_config = &AppConfiguration::new();
    let puzzle_dir = app_config.puzzle_dir.clone();
    configure_flexi_logger(app_config.clone().log_spec).expect("Failed to configure logger - quitting");
    log::info!("Starting process");

    let cli = Cli::parse();
    match &cli.commands {
        Commands::Scratch {} => scratch(),
        Commands::Day1 {} => {
            let day = Day::new(1, puzzle_dir);
            let answer = crate::day_1::solver::solve(&day.data_path);
            log::info!("Solved {}: answer={}", day.label(), answer);
        }
        Commands::Day2 {} => {
            let day = Day::new(2, puzzle_dir);
            let answer = day_2::solver::solve_pt_1(&day.data_path);
            log::info!("Solved {} part 1: answer={}", day.label(), answer);
            let answer = day_2::solver::solve_pt_2(&day.data_path);
            log::info!("Solved {} part 2: answer={}", day.label(), answer);
        }
        Commands::Day3 {} => {
            let day = Day::new(3, puzzle_dir);
            let answer = day_3::solver::solve_pt_1(&day.data_path);
            log::info!("Solved {} part 1: answer={}", day.label(), answer);
            let answer = day_3::solver::solve_pt_2(&day.data_path);
            log::info!("Solved {} part 2: answer={}", day.label(), answer);
        }
        Commands::Day4 {} => {
            let day = Day::new(4, puzzle_dir);
            let answer = day_4::solver::solve_pt_1(&day.data_path);
            log::info!("Solved {} part 1: answer={}", day.label(), answer);
            let answer = day_4::solver::solve_pt_2(&day.data_path);
            log::info!("Solved {} part 2: answer={}", day.label(), answer);
        }
        Commands::Day5 {} => {
            let day = Day::new(5, puzzle_dir);
            let answer = day_5::solver::solve_pt_1(&day.data_path);
            log::info!("Solved {} part 1: answer={}", day.label(), answer);
            let answer = day_5::solver::solve_pt_2(&day.data_path);
            log::info!("Solved {} part 2: answer={}", day.label(), answer);
        }
        Commands::Day6 {} => {
            let day = Day::new(6, puzzle_dir);
            let answer = day_6::solver::solve_pt_1(&day.data_path);
            log::info!("Solved {} part 1: answer={}", day.label(), answer);
            let answer = day_6::solver::solve_pt_2(&day.data_path);
            log::info!("Solved {} part 2: answer={}", day.label(), answer);
        }
        Commands::Day7 {} => {
            let day = Day::new(7, puzzle_dir);
            let answer = day_7::solver::solve_pt_1(&day.data_path);
            log::info!("Solved {} part 1: answer={}", day.label(), answer);
            let answer = day_7::solver::solve_pt_2(&day.data_path);
            log::info!("Solved {} part 2: answer={}", day.label(), answer);
        }
        Commands::Day8 {} => {
            let day = Day::new(8, puzzle_dir);
            let answer = day_8::solver::solve_pt_1(&day.test_data_path, 10);
            log::info!("Solved {} part 1: answer={}", day.label(), answer);
            let answer = day_8::solver::solve_pt_2(&day.test_data_path);
            log::info!("Solved {} part 2: answer={}", day.label(), answer);
        }
        Commands::Day9 {} => {
            let day = Day::new(9, puzzle_dir);
            let answer = day_9::solver::solve_pt_1(&day.data_path);
            log::info!("Solved {} part 1: answer={}", day.label(), answer);
            let answer = day_9::solver::solve_pt_2(&day.data_path);
            log::info!("Solved {} part 2: answer={}", day.label(), answer);
        }
        Commands::Day10 {} => {
            let day = Day::new(10, puzzle_dir);
            let answer = day_10::solver::solve_pt_1(&day.test_data_path);
            log::info!("Solved {} part 1: answer={}", day.label(), answer);
            let answer = day_10::solver::solve_pt_2(&day.test_data_path);
            log::info!("Solved {} part 2: answer={}", day.label(), answer);
        }
    }
    log::info!("Process Completed");
}
