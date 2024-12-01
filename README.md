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
cargo install just cargo-generate cargo-watch
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

### 3. Build the workspace

```sh
cargo build
```

## Daily Setup
