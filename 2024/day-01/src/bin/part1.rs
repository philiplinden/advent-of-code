use day_01::part1::process;

use miette::Context;

fn main() -> miette::Result<()> {
    aoc_tools::load_env();
    let input = include_str!("../../input1.txt");

    let result = process(&input).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
