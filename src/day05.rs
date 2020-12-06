use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryFrom;

struct Seat {
    row: usize,
    column: usize,
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Seat> {
    input
        .lines()
        .map(|s| {
            let replaced = {
                s.trim()
                    .replace(|c| c == 'B' || c == 'R', "1")
                    .replace(|c| c == 'F' || c == 'L', "0")
            };

            Seat {
                row: usize::from_str_radix(&replaced[..7], 2).unwrap(),
                column: usize::from_str_radix(&replaced[7..], 2).unwrap(),
            }
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(vals: &[Seat]) -> usize {
    vals.iter().map(|s| s.row * 8 + s.column).max().unwrap()
}

#[aoc(day5, part2)]
fn part2(vals: &[Seat]) -> usize {
    let mut ids: Vec<usize> = vals.iter().map(|s| s.row * 8 + s.column).collect();
    ids.sort_unstable();

    for [a, b] in ids.windows(2).map(|s| <[usize; 2]>::try_from(s).unwrap()) {
        if b - a == 2 {
            return b - 1;
        }
    }

    unreachable!()
}
