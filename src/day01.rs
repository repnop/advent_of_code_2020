use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|s| s.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(vals: &[usize]) -> usize {
    vals.iter()
        .copied()
        .combinations(2)
        .find(|v| v[0] + v[1] == 2020)
        .map(|v| v[0] * v[1])
        .unwrap()
}

#[aoc(day1, part2)]
fn part2(vals: &[usize]) -> usize {
    vals.iter()
        .copied()
        .combinations(3)
        .find(|v| v[0] + v[1] + v[2] == 2020)
        .map(|v| v[0] * v[1] * v[2])
        .unwrap()
}
