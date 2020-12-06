use aoc_runner_derive::{aoc, aoc_generator};
use regex::bytes::Regex;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<HashMap<String, String>> {
    input
        .split("\n\n")
        .map(|s| s.replace('\n', " "))
        .map(|s| {
            s.split(' ')
                .map(|s| {
                    (
                        s.split(':').next().unwrap().to_string(),
                        s.split(':').nth(1).unwrap().to_string(),
                    )
                })
                .collect()
        })
        .collect()
}

// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)

#[aoc(day4, part1)]
fn part1(vals: &[HashMap<String, String>]) -> usize {
    vals.iter()
        .map(|m| -> HashSet<&str> { m.keys().map(String::as_str).collect() })
        .filter(|p| {
            p.contains("byr")
                && p.contains("iyr")
                && p.contains("eyr")
                && p.contains("hgt")
                && p.contains("hcl")
                && p.contains("ecl")
                && p.contains("pid")
        })
        .count()
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
//     If cm, the number must be at least 150 and at most 193.
//     If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.

#[aoc(day4, part2)]
fn part2(vals: &[HashMap<String, String>]) -> usize {
    let hair_color_regex = Regex::new("#[0-9a-f]{6}").unwrap();
    vals.iter()
        .filter_map(|m| {
            let height_val = m.get("hgt")?;
            let pid = m.get("pid")?;
            if (1920..=2002).contains(&m.get("byr")?.parse::<u32>().ok()?)
                && (2010..=2020).contains(&m.get("iyr")?.parse::<u32>().ok()?)
                && (2020..=2030).contains(&m.get("eyr")?.parse::<u32>().ok()?)
                && match &height_val[height_val.len() - 2..height_val.len()] {
                    "in" => {
                        (59..=76).contains(&height_val.trim_end_matches("in").parse::<u32>().ok()?)
                    }
                    "cm" => (150..=193)
                        .contains(&height_val.trim_end_matches("cm").parse::<u32>().ok()?),
                    _ => return None,
                }
                && hair_color_regex.is_match(m.get("hcl")?.as_bytes())
                && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&m.get("ecl")?)
                && pid.len() == 9
                && pid.chars().all(|c| c.is_ascii_digit())
            {
                Some(())
            } else {
                None
            }
        })
        .count()
}
