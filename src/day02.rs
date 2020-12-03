use core::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use bstr::{BString, ByteSlice};
use once_cell::sync::Lazy;
use regex::Regex;

static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("(\\d+)-(\\d+) (.): (.+)").unwrap());

struct PasswordEntry {
    range: RangeInclusive<usize>,
    character: char,
    password: BString,
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<PasswordEntry> {
    input
        .lines()
        .map(|s| {
            let caps = REGEX.captures(s).unwrap();
            PasswordEntry {
                range: caps[1].parse().unwrap()..=caps[2].parse().unwrap(),
                character: caps[3].chars().next().unwrap(),
                password: caps[4].into(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(vals: &[PasswordEntry]) -> usize {
    vals.iter()
        .filter(
            |PasswordEntry {
                 range: r,
                 character: c,
                 password: p,
             }| r.contains(&p.chars().filter(|c2| c2 == c).count()),
        )
        .count()
}

#[aoc(day2, part2)]
fn part2(vals: &[PasswordEntry]) -> usize {
    vals.iter()
        .filter(
            |PasswordEntry {
                 range: r,
                 character: c,
                 password: p,
             }| (p[*r.start() - 1] == *c as u8) ^ (p[*r.end() - 1] == *c as u8),
        )
        .count()
}
