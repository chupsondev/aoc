# Advent of Code
This repository contains my solutions to whatever [AoC](https://adventofcode.com/) puzzles I wanted and was able to solve.

# Templates
The templates that I use for my solutions
## Rust
```rust
use std::fs;

mod part1 {
    pub fn solve(input: &str) -> i32 {
        todo!()
    }
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    println!("Part 1: {}", part1::solve(&input));
}

#[cfg(test)]
mod pt1_tests {
    use super::*;

    #[test]
    fn example() {}
}
```
