// src/main.rs
use clap::{Arg, ArgMatches, Command};
use std::process::Command as ShellCommand;

fn main() {
    let matches = Command::new("Advent of Code CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Streamlined build and development workflow for Advent of Code")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("work")
                .about("Watch and test a specific day's puzzle")
                .arg(Arg::new("day").required(true).help("Day (e.g., day-01)"))
                .arg(Arg::new("part").required(true).help("Part (e.g., part1)")),
        )
        .subcommand(
            Command::new("test")
                .about("Run tests for a specific day and part")
                .arg(Arg::new("day").required(true).help("Day (e.g., day-01)"))
                .arg(Arg::new("part").required(true).help("Part (e.g., part1)")),
        )
        .subcommand(
            Command::new("bench-all")
                .about("Run all benchmarks"),
        )
        .subcommand(
            Command::new("bench")
                .about("Run benchmark for a specific day and part")
                .arg(Arg::new("day").required(true).help("Day (e.g., day-01)"))
                .arg(Arg::new("part").required(true).help("Part (e.g., part1)")),
        )
        .subcommand(
            Command::new("create")
                .about("Create a new day's puzzle directory")
                .arg(Arg::new("day").required(true).help("Day (e.g., day-01)")),
        )
        .subcommand(
            Command::new("get-input")
                .about("Fetch puzzle input for a specific day")
                .arg(Arg::new("day").required(true).help("Day (e.g., day-01)")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("work", sub_m)) => handle_work(sub_m),
        Some(("test", sub_m)) => handle_test(sub_m),
        Some(("bench-all", _)) => handle_bench_all(),
        Some(("bench", sub_m)) => handle_bench(sub_m),
        Some(("create", sub_m)) => handle_create(sub_m),
        Some(("get-input", sub_m)) => handle_get_input(sub_m),
        _ => {}
    }
}

fn handle_work(matches: &ArgMatches) {
    let day = matches.get_one::<String>("day").expect("day is required");
    let part = matches.get_one::<String>("part").expect("part is required");

    ShellCommand::new("cargo")
        .args(&["watch", "-w", day, "-x", "check", "-p", day, "-s", "test", "DAY=day", "PART=part"])
        .status()
        .expect("Failed to execute work command");
}

fn handle_test(matches: &ArgMatches) {
    let day = matches.get_one::<String>("day").expect("day is required");
    let part = matches.get_one::<String>("part").expect("part is required");

    ShellCommand::new("cargo")
        .args(&["nextest", "run", "-p", day, part])
        .status()
        .expect("Failed to execute test command");
}

fn handle_bench_all() {
    ShellCommand::new("cargo")
        .args(&["bench", "-q"])
        .output()
        .expect("Failed to execute bench-all command");
}

fn handle_bench(matches: &ArgMatches) {
    let day = matches.get_one::<String>("day").expect("day is required");
    let part = matches.get_one::<String>("part").expect("part is required");

    ShellCommand::new("cargo")
        .args(&["bench", "--bench", &format!("{}-bench", day), part])
        .output()
        .expect("Failed to execute bench command");
}

fn handle_create(matches: &ArgMatches) {
    let day = matches.get_one::<String>("day").expect("day is required");

    ShellCommand::new("cargo")
        .args(&["generate", "--path", "./daily-template", "--name", day])
        .status()
        .expect("Failed to execute create command");

    handle_get_input(matches);
}

fn handle_get_input(matches: &ArgMatches) {
    let day = matches.get_one::<String>("day").expect("day is required");

    ShellCommand::new("./scripts/get-input.sh")
        .args(&["--day", day, "--current-working-directory", &std::env::current_dir().unwrap().to_string_lossy()])
        .status()
        .expect("Failed to execute get-input command");
}
