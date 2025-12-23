use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "aoc2025", version)]
pub struct Cli {
    #[clap(flatten)]
    pub global_opts: GlobalOpts,

    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Args)]
pub struct GlobalOpts {

}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Scratch {},
    Day1 {},
    Day2 {},
    Day3 {},
    Day4 {},
    Day5 {},
    Day6 {},
    Day7 {},
    Day8 {},
    Day9 {},
    Day10 {},
}

