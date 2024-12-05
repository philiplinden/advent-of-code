use day_01::*;

fn main() -> miette::Result<()> {
    // Run registered benchmarks.
    divan::main();
    Ok(())
}

#[divan::bench]
fn part1() -> miette::Result<()> {
    part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))?;
    Ok(())
}

#[divan::bench]
fn part2() -> miette::Result<()> {
    part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))?;
    Ok(())
}
