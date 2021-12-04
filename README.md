# Chance of Elimination.
Monte-Carlo simulation to determine approximate chances of elimination for athletes in point-based series.

## Building.
Building is just like any other [Rust](https://www.rust-lang.org/) project. Run `cargo run --release` in the root of the repository. The build will be outputted in the `target` directory.

## Usage.
Run the executable in the same directory as the `data` directory. The `data` directory should have the `athletes.toml` file contained within. The `athletes.toml` file contains instructions on what each field means. Edit this file with your required values and run the executable.
