use chrono::{Datelike, Local};
use clap::{Args, Parser, Subcommand};
use std::{path::PathBuf, process::Command as ShellCommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    job: Jobs,

    #[command(flatten)]
    args: CommonArgs,
}

#[derive(Args, Debug, Clone)]
struct CommonArgs {
    #[arg(long)]
    year: Option<u32>,

    #[arg(long)]
    day: Option<u32>,

    #[arg(long)]
    part: Option<u32>,
}

fn main() {
    let cli = Cli::parse();
    cli.job.run(&cli.args);
}

// =================== JOBS ====================================================

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
    fn run(&self, args: &CommonArgs) {
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
            Self::BenchAll => handle_bench_all(),
            _ => unreachable!(),
        }
    }
}

fn get_day_dir(year: u32, day: u32) -> PathBuf {
    PathBuf::from(format!("{}/day-{:02}", year, day))
}

fn handle_new(year: u32, day: u32) {
    let day_dir = get_day_dir(year, day);
    let day_name = day_dir.file_name().unwrap().to_str().unwrap();

    // Create the year directory if it doesn't exist
    std::fs::create_dir_all(year.to_string()).expect("Failed to create year directory");

    ShellCommand::new("cargo")
        .args(&["generate", "--path", "./daily-template", "--name", day_name])
        .status()
        .expect("Failed to execute create command");

    // Move the generated directory to the year folder
    if day_dir.exists() {
        std::fs::remove_dir_all(&day_dir).expect("Failed to remove existing day directory");
    }
    std::fs::rename(day_name, &day_dir).expect("Failed to move day directory to year folder");

    handle_fetch(year, day);
}

fn handle_fetch(year: u32, day: u32) {
    let session = std::env::var("SESSION").expect("SESSION environment variable not set");
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let response = reqwest::blocking::get(&url)
        .expect("Failed to send request")
        .text()
        .expect("Failed to read response text");

    let day_dir = get_day_dir(year, day);
    std::fs::create_dir_all(&day_dir).expect("Failed to create day directory");
    let input_path = day_dir.join("input.txt");
    std::fs::write(&input_path, response).expect("Failed to write input file");

    println!("Input for day {} saved to {}", day, input_path.display());
}

fn handle_test(year: u32, day: u32, part: u32) {
    let day_dir = get_day_dir(year, day);
    let day_name = day_dir.file_name().unwrap().to_str().unwrap();

    ShellCommand::new("cargo")
        .args(&["nextest", "run", "-p", day_name, &part.to_string()])
        .current_dir(year.to_string())
        .status()
        .expect("Failed to execute test command");
}

fn handle_bench(year: u32, day: u32, part: u32) {
    let day_dir = get_day_dir(year, day);
    let day_name = day_dir.file_name().unwrap().to_str().unwrap();

    ShellCommand::new("cargo")
        .args(&[
            "bench",
            "--bench",
            &format!("{}-bench", day_name),
            &part.to_string(),
        ])
        .current_dir(year.to_string())
        .output()
        .expect("Failed to execute bench command");
}

fn handle_bench_all() {
    // Find all year directories
    for entry in std::fs::read_dir(".").expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_dir() {
            if let Some(year_str) = path.file_name().and_then(|n| n.to_str()) {
                if let Ok(_year) = year_str.parse::<u32>() {
                    ShellCommand::new("cargo")
                        .args(&["bench", "-q"])
                        .current_dir(path)
                        .output()
                        .expect("Failed to execute bench command");
                }
            }
        }
    }
}
