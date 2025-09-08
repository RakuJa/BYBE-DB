![Rust](https://img.shields.io/badge/Rust-664666?style=for-the-badge&logo=rust&logoColor=red)
![SQLite](https://img.shields.io/badge/sqlite-%2307405e.svg?style=for-the-badge&logo=sqlite&logoColor=white)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)


# BYBE - DB

> Beyond Your Bestiary Explorer (BYBE) provides tools to help Pathfinder 2e Game Masters. Built as the database initializer of [BYBE - Backend](https://github.com/RakuJa/BYBE/)

## Features

This rust program automagically downloads foundry data (and it's kinda slow, beware!) and then parses it storing in a more organized form in a SQLite DB.

## Requirements

Built using:

- [Rust](https://www.rust-lang.org/tools/install)
- [SQLite](https://www.sqlite.org/download.html)

## Installation guide - Local

1. Install [Rust](https://www.rust-lang.org/tools/install) on your machine.
2. Setup .env correctly (for sane defaults rename .env.example -> .env)

3. Build the project:

```bash
cargo build
```
4. Run the database initializer in development mode:

```bash
cargo run
```

To instead deploy the production build, run:

```bash
cargo build --release
```

```bash
cargo run
```
It should be quick (~1 minute) if the foundry data is already cloned, otherwise it can last a long time.
## Support me

If you like this tool, consider supporting me:

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/rakuja)
