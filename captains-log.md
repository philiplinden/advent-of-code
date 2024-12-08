# captain's log

## 2024-12-08

Ha, so I'm still on day 1. I'd rather finish one puzzle before moving to the
next one. I'm learning a lot and proud of my progress so far.

Finally finished day 1. I'm not sure I'll do the rest of the puzzles this year,
but I'm glad I stuck with it until I finished at least one.

I learned a lot about Rust and learned that parallelization is simpler to
implement than I thought, but it's not always faster.

This is (embarrassingly) the first time I've used tests. It worked out great. I
wrote the tests while writing the [`README.md`](2024/day-01/README.md) and
wrote them to prove the advent of code's example worked with my code.

Benchmarks are also new to me. I used [divan](https://github.com/orium/divan)
to benchmark the two implementations of part 1. It worked great.

## 2024-12-05

I'm on a cleaning streak. I'm going through and deleting all the pieces of the
templates and CLI that I don't use.

Now, before I even start on the day 1 problem I need to learn how to parse input
files. The `nom` crate looks cool, but it's definitely overkill for this. I'll
practice my skills with the std library first.

## 2024-12-04

I finished the CLI and added a few more features. I added the ability to fetch
input data for a specific year and/or day and save it to a specific directory.
The `new` command will create a new project for the current year and day and
generate a new project from [the template](aoc-cli/src/daily-template).

I finally started day 1 -> [day-01 log](2024/day-01/README.md).

I'm going for speed. Beyond solving the problem, I want to see how fast I can
get the code to run.

## 2024-12-01

Trying out advent of code for the first time. I'm trying out some tooling
from
[ChristopherBiscardi/advent-of-code](https://github.com/ChristopherBiscardi/advent-of-code),
including [just](https://github.com/casey/just) for the first time.

I use `nushell` as my shell, so I had to modify the `justfile` to work with it.
I ended up making a few more changes to get it working the way I wanted. I added
setup instructions to the README.

This setup is a pain in the ass, but it's a good excercise in tooling. I'm going
through and recreating all the tooling in shell and python that I understand.
I could probably reuse some of this as a template for game jams and the like.

I experimented with nushell for a bit, but I think I'll spend some time making a
proper CLI tool in rust or python. I'll start with rust for practice.

I made the rust CLI and like it a lot so far.
