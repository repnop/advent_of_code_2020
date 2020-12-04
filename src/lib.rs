mod day01;
mod day02;
mod day03;
mod day04;

aoc_runner_derive::aoc_lib! { year = 2020 }

macro_rules! regex_deserialize {
    ($re:literal => struct $s:ident { $($f:ident: $t:ty),+$(,)? }) => {
        struct $s {
            $($f: $t),+
        }

        impl $s {
            pub fn deserialize(s: &str) -> Option<Self> {
                static REGEX: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| regex::Regex::new($re).unwrap());
                let captures = REGEX.captures(s)?;

                Some(Self {
                    $($f: captures.get(stringify!($f))?.as_str().parse::<$t>().ok()?),+
                })
            }
        }
    }
}
