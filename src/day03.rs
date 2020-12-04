use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
enum Tile {
    Tree,
    Clear,
}

struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Board {
    fn tile_at(&self, x: usize, y: usize) -> Tile {
        self.tiles[self.width * y + (x % self.width)]
    }

    fn tree_count_from_slope(&self, dx: usize, dy: usize) -> usize {
        let (mut x, mut y) = (0, 0);
        let mut tree_count = 0;

        while y < self.height {
            if let Tile::Tree = self.tile_at(x, y) {
                tree_count += 1;
            }

            x += dx;
            y += dy;
        }

        tree_count
    }
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Board {
    let height = input.lines().count();
    let tiles: Vec<Tile> = input
        .lines()
        .map(|l| l.chars())
        .flatten()
        .map(|c| match c {
            '#' => Tile::Tree,
            '.' => Tile::Clear,
            _ => unreachable!(),
        })
        .collect();
    let width = tiles.len() / height;

    Board {
        width,
        height,
        tiles,
    }
}

#[aoc(day3, part1)]
fn part1(board: &Board) -> usize {
    board.tree_count_from_slope(3, 1)
}

#[aoc(day3, part2)]
fn part2(board: &Board) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, (dx, dy)| {
            acc * board.tree_count_from_slope(*dx, *dy)
        })
}
