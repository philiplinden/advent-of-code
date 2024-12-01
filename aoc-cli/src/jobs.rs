use chrono::{Local, Datelike};
use clap::Subcommand;
use std::process::Command as ShellCommand;
use crate::CommonArgs;
use std::path::PathBuf;

#[derive(Subcommand, Debug, Clone)]
pub enum Jobs {
    New {
        #[arg(long)]
        year: Option<u32>,
        #[arg(long)]
        day: Option<u32>,
    },
    Fetch {
        #[arg(long)]
        year: Option<u32>,
        #[arg(long)]
        day: Option<u32>,
        #[arg(long)]
        directory: Option<PathBuf>,
    },
    Test {
        #[arg(long)]
        year: Option<u32>,
        #[arg(long)]
        day: Option<u32>,
        #[arg(long)]
        part: Option<u32>,
    },
    Bench {
        #[arg(long)]
        year: Option<u32>,
        #[arg(long)]
        day: Option<u32>,
        #[arg(long)]
        part: Option<u32>,
    },
    BenchAll,
}

impl Jobs {
    pub fn run(&self, args: &CommonArgs) {
        let (year, day, part) = {
            let year = args.year.unwrap_or_else(|| Local::now().year() as u32);
            let day = args.day.unwrap_or_else(|| Local::now().day() as u32);
            let part = args.part.unwrap_or(1);
            (year, day, part)
        };

        match self {
            Self::New { .. } => handle_new(year, day),
            Self::Fetch { .. } => handle_fetch(year, day),
            Self::Test { .. } => handle_test(year, day, part),
            Self::Bench { .. } => handle_bench(year, day, part),
            _ => unreachable!(),
        }
    }
}

fn handle_new(year: u32, day: u32) {
    let day_str = format!("day-{:02}", day);
    ShellCommand::new("cargo")
        .args(&["generate", "--path", "./daily-template", "--name", &day_str])
        .status()
        .expect("Failed to execute create command");

    handle_fetch(year, day);
}

fn handle_fetch(year: u32, day: u32) {
    let session = std::env::var("SESSION").expect("SESSION environment variable not set");
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    
    let response = reqwest::blocking::get(&url)
        .expect("Failed to send request")
        .text()
        .expect("Failed to read response text");
    
    let day_dir = format!("day-{:02}", day);
    std::fs::create_dir_all(&day_dir).expect("Failed to create day directory");
    let input_path = format!("{}/input.txt", day_dir);
    std::fs::write(&input_path, response).expect("Failed to write input file");
    
    println!("Input for day {} saved to {}", day, input_path);
}

fn handle_test(year: u32, day: u32, part: u32) {
    let day_str = format!("day-{:02}", day);
    ShellCommand::new("cargo")
        .args(&["nextest", "run", "-p", &day_str, &part.to_string()])
        .status()
        .expect("Failed to execute test command");
}

fn handle_bench(year: u32, day: u32, part: u32) {
    let day_str = format!("day-{:02}", day);
    ShellCommand::new("cargo")
        .args(&["bench", "--bench", &format!("{}-bench", day_str), &part.to_string()])
        .output()
        .expect("Failed to execute bench command");
}

fn handle_bench_all() {
    ShellCommand::new("cargo")
        .args(&["bench", "-q"])
        .output()
        .expect("Failed to execute bench-all command");
}
