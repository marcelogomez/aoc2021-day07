// Advent of Code 2021, Day 7
// https://adventofcode.com/2021/day/7

use std::{io::BufRead, str::FromStr};

#[derive(Debug, Copy, Clone)]
struct Solution {
    pub dest: u32,
    pub distance_sum: u32,
}

fn parse_input_line(s: &str) -> anyhow::Result<Vec<i32>> {
    Ok(s.trim()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(i32::from_str)
        .collect::<Result<_, _>>()?)
}

fn calculate_distance_sum<F>(dest: i32, positions: &[i32], distance_fn: F) -> u32
where
    F: Fn(i32, i32) -> u32,
{
    positions
        .iter()
        .map(|position| distance_fn(dest, *position))
        .sum()
}

fn find_min_distance_sum<F>(positions: &[i32], distance_fn: F) -> anyhow::Result<Solution>
where
    F: Fn(i32, i32) -> u32 + Clone,
{
    let start = *positions
        .iter()
        .min()
        .ok_or_else(|| anyhow::anyhow!("expected at least one position"))?;
    let end = *positions
        .iter()
        .max()
        .ok_or_else(|| anyhow::anyhow!("expected at least one position"))?;

    (start..end)
        .map(|dest| {
            (
                dest as u32,
                // TODO: Figure out how to avoid this clone
                calculate_distance_sum(dest, positions, distance_fn.clone()),
            )
        })
        .min_by(|sol_1, sol_2| sol_1.1.cmp(&sol_2.1))
        .map(|(dest, distance_sum)| Solution { dest, distance_sum })
        .ok_or_else(|| anyhow::anyhow!("expected at least one position"))
}

fn part_1_distance(pos1: i32, pos2: i32) -> u32 {
    (pos1 - pos2).abs() as u32
}

fn solve_part_1(positions: &[i32]) -> anyhow::Result<Solution> {
    find_min_distance_sum(positions, part_1_distance)
}

fn main_impl() -> anyhow::Result<()> {
    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line)?;

    let positions = parse_input_line(&line)?;

    println!("Part 1 solution {:#?}", solve_part_1(&positions));

    Ok(())
}

fn main() {
    main_impl().unwrap()
}
