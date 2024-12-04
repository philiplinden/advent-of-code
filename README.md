# Advent of Code

Solutions for the [Advent of Code](http://adventofcode.com/)

Borrows some tooling from
[ChristopherBiscardi/advent-of-code](https://github.com/ChristopherBiscardi/advent-of-code).

## Workspace Setup

### 1. Install dependencies

- [rust](https://www.rust-lang.org/tools/install)
- [cargo-generate](https://github.com/cargo-generate/cargo-generate)
- [cargo-watch](https://github.com/watchexec/cargo-watch)

```sh
cargo install cargo-generate cargo-watch
```

### 2. Create `.env` file

Go to [adventofcode.com](https://adventofcode.com/), log in, and grab the value
for `session` from the cookie.

1. Open the developer tools (F12)
2. Go to the "Application" tab
3. Go to "Cookies"
4. Find the `session` cookie

```sh
echo "SESSION=PASTE_COOKIE_VALUE_HERE" > .env
```

### 3. (Optional) Build & install the CLI helper

```sh
cargo install --path aoc-cli --release
```

## Daily Setup

The `new` command will create a new project for the current year and day and
generate a new project from [the template](aoc-cli/src/daily-template).

```sh
aoc new
```

To pull the input data for a specific year and/or day when generating a new
project from the template:

```sh
aoc new --year 2024 --day 1
```

Or to pull the input data for a specific year and/or day and save it to a
specific directory:

```sh
aoc fetch --year 2024 --day 1 --output-directory ./my_happy_place
```
