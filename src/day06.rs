use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<(usize, HashMap<char, usize>)> {
    input
        .split("\n\n")
        .map(|s| {
            (
                s.chars().filter(|c| *c == '\n').count() + 1,
                s.chars()
                    .filter(|c| !c.is_whitespace())
                    .fold(HashMap::new(), |mut hm, c| {
                        *hm.entry(c).or_insert(0) += 1;
                        hm
                    }),
            )
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(vals: &[(usize, HashMap<char, usize>)]) -> usize {
    vals.iter().map(|(_, h)| h.keys().len()).sum()
}

#[aoc(day6, part2)]
fn part2(vals: &[(usize, HashMap<char, usize>)]) -> usize {
    vals.iter()
        .map(|(group_size, h)| h.iter().filter(|(_, n)| *n == group_size).count())
        .sum()
}

#[test]
fn part_2_example() {
    let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    assert_eq!(part2(&parse_input(input)), 6);
}
