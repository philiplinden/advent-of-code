use day_01::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1_sequential() {
    part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )), false)
    .unwrap();
}

#[divan::bench]
fn part1_parallel() {
    part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )), true)
    .unwrap();
}

// #[divan::bench]
// fn part2() {
//     part2::process(divan::black_box(include_str!(
//         "../input2.txt",
//     )))
//     .unwrap();
// }
