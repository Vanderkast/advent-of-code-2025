use crate::day_1::DayOne;
use crate::day_2::DayTwo;
use clap::Parser;
use clap_derive::{Subcommand, ValueEnum};
use std::fmt::Formatter;
use std::path::PathBuf;
use std::process::exit;

mod day_1;
mod day_2;

#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    day: Day,
}

#[derive(Clone, Debug, Subcommand)]
enum Day {
    DayOne {
        part: Part,
        #[arg(
            short,
            long,
            value_parser = clap::value_parser!(PathBuf),
            default_value = first_day_default_input_path().into_os_string(),
        )]
        path: PathBuf,
    },
    DayTwo {
        part: Part,
        #[arg(
            short,
            long,
            value_parser = clap::value_parser!(PathBuf),
            default_value = second_day_default_input_path().into_os_string(),
        )]
        path: PathBuf,
    },
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Day::DayOne { .. } => write!(f, "1"),
            Day::DayTwo { .. } => write!(f, "2"),
        }
    }
}

fn first_day_default_input_path() -> PathBuf {
    PathBuf::from("./inputs/day_1.txt")
}

fn second_day_default_input_path() -> PathBuf {
    PathBuf::from("./inputs/day_2.txt")
}

#[derive(Clone, Debug, ValueEnum)]
enum Part {
    PartOne,
    PartTwo,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::PartOne => write!(f, "1"),
            Part::PartTwo => write!(f, "2"),
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);

    let result = match args.day {
        Day::DayOne { part, path } => DayOne { part, path }.run(),
        Day::DayTwo { part, path } => DayTwo::new(part, path).run(),
    };

    match result {
        Ok(msg) => {
            println!("{}", msg);
        }
        Err(msg) => {
            eprintln!("{}", msg);
            exit(1);
        }
    }
}
