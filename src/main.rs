// Advent of Code 2021, Day 7
// https://adventofcode.com/2021/day/7

use std::{io::BufRead, str::FromStr};

fn parse_input_line(s: &str) -> anyhow::Result<Vec<u32>> {
    Ok(s.split(',').map(u32::from_str).collect::<Result<_, _>>()?)
}

fn main_impl() -> anyhow::Result<()> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;

    let positions = parse_input_line(&line)?;

    println!("{:?}", positions);

    Ok(())
}

fn main() {
    main_impl().unwrap()
}
