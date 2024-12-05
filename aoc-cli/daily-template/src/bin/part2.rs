use {{crate_name}}::part2::process;
use miette::Context;

fn main() -> miette::Result<()> {
    aoc_tools::load_env();
    let input = include_str!("../../input2.txt");

    let result = process(&input).context("process part 2")?;
    println!("{}", result);
    Ok(())
}
