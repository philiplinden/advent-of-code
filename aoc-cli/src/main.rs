use chrono::{Datelike, Local};
use clap::{Parser, Subcommand};
use include_dir::{include_dir, Dir};
use log::{error, info};
use reqwest::{blocking::Client, header::COOKIE};
use std::{
    fs::File,
    io::{Error, Write},
    path::PathBuf,
    process::Command as ShellCommand,
};

static TEMPLATE_PATH: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/daily-template");

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about = "Streamlined build and development CLI for Advent of Code",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    job: Jobs,
}

fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();
    let cli = Cli::parse();
    let (default_year, default_day, default_part) = {
        let year = Local::now().year() as u32;
        let day = Local::now().day() as u32;
        let part = 1;
        (year, day, part)
    };
    match cli.job {
        Jobs::New { year, day } => {
            handle_new(year.unwrap_or(default_year), day.unwrap_or(default_day))
        }
        Jobs::Fetch { year, day, output_directory } => {
            handle_fetch(year.unwrap_or(default_year), day.unwrap_or(default_day), output_directory)
        }
        Jobs::Test { year, day, part } => handle_test(
            year.unwrap_or(default_year),
            day.unwrap_or(default_day),
            part.unwrap_or(default_part),
        ),
        Jobs::Bench { year, day, part } => handle_bench(
            year.unwrap_or(default_year),
            day.unwrap_or(default_day),
            part.unwrap_or(default_part),
        ),
        Jobs::BenchAll => handle_bench_all(),
    }
}

// =================== JOBS ====================================================

#[derive(Subcommand, Debug, Clone)]
pub enum Jobs {
    /// Creates a new Advent of Code project for the specified year and day.
    New {
        #[arg(short, long)]
        year: Option<u32>,
        #[arg(short, long)]
        day: Option<u32>,
    },
    /// Fetches the input data for the specified year and day from the Advent of Code website.
    Fetch {
        #[arg(short, long)]
        year: Option<u32>,
        #[arg(short, long)]
        day: Option<u32>,
        #[arg(short, long)]
        output_directory: Option<PathBuf>,
    },
    /// Runs tests for the specified year, day, and part of the Advent of Code challenge.
    Test {
        #[arg(short, long)]
        year: Option<u32>,
        #[arg(short, long)]
        day: Option<u32>,
        #[arg(short, long)]
        part: Option<u32>,
    },
    /// Benchmarks the specified year, day, and part of the Advent of Code challenge.
    Bench {
        #[arg(short, long)]
        year: Option<u32>,
        #[arg(short, long)]
        day: Option<u32>,
        #[arg(short, long)]
        part: Option<u32>,
    },
    /// Benchmarks all Advent of Code projects across all years.
    BenchAll,
}

fn get_env_var(key: &str) -> Option<String> {
    match std::env::var(key) {
        Ok(val) => Some(val),
        Err(_) => {
            error!("{} environment variable not set", key);
            None
        }
    }
}

fn get_year_dir(year: u32) -> PathBuf {
    let year_dir = PathBuf::from(format!("{}", year));
    if !year_dir.exists() {
        std::fs::create_dir_all(&year_dir).expect("Failed to create year directory");
    }
    year_dir
}

fn get_day_dir(year: u32, day: u32) -> PathBuf {
    let year_dir = get_year_dir(year);
    let day_dir = year_dir.join(format!("day-{:02}", day));
    if !day_dir.exists() {
        std::fs::create_dir_all(&day_dir).expect("Failed to create day directory");
    }
    day_dir
}

fn unpack_template() -> Result<PathBuf, std::io::Error> {
    let temp_dir = tempfile::tempdir()?;
    TEMPLATE_PATH.extract(&temp_dir.path())?;
    Ok(temp_dir.into_path())
}

fn run_cargo_generate(template_dir: &PathBuf, destination: &PathBuf, name: &str) -> Result<(), Error> {
    let project_directory = destination.join(name);
    if project_directory.exists() && project_directory.read_dir()?.count() > 0 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            format!("destination path is not empty: {}", project_directory.display()),
        ));
    }
    let output = ShellCommand::new("cargo")
        .arg("generate")
        .arg("--verbose")
        .arg("--path")
        .arg(&template_dir)
        .arg("--destination")
        .arg(destination)
        .arg("--name")
        .arg(name)
        .output()?;

    if output.status.success() {
        info!("cargo-generate succeeded");
        Ok(())
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr),
        ))
    }
}

fn handle_new(year: u32, day: u32) {
    let template_dir = unpack_template().expect("Failed to unpack template");
    let destination = get_year_dir(year);
    match run_cargo_generate(&template_dir, &destination, &format!("day-{:02}", day)) {
        Ok(_) => handle_fetch(year, day, None),
        Err(e) => {
            error!("Failed to run cargo generate: {}", e);
            error!("If you meant to just get the input data, use the fetch command instead.");
        }
    }
}

fn handle_fetch(year: u32, day: u32, output_directory: Option<PathBuf>) {
    let Some(session) = get_env_var("SESSION") else {
        error!("SESSION environment variable not set");
        return;
    };
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let destination = if let Some(output_directory) = output_directory {
        output_directory.join(format!("day-{:02}", day))
    } else {
        get_day_dir(year, day)
    };

    let client = Client::new();
    let input_data = client
        .get(url)
        .header(COOKIE, format!("session={session}"))
        .send()
        .expect("Failed to send request")
        .text()
        .expect("Failed to read response text");

    for filename in ["input1.txt", "input2.txt"] {
        let file_path = destination.join(filename);
        let mut file = File::create(&file_path).expect("should be able to create a file");

        file.write_all(input_data.as_bytes())
            .expect("should be able to write to input file");
        info!("wrote {}", file_path.display());
    }
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
