use itertools::Itertools;
use lazy_static::lazy_static;
use std::fmt::Display;

lazy_static! {
    static ref INPUT: Vec<u32> = include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
}

fn main() {
    println!("*  {}", part1());
    println!("** {}", part2());
}

fn part1() -> impl Display {
    INPUT.iter().tuple_windows().filter(|(x, y)| y > x).count()
}

fn part2() -> impl Display {
    INPUT
        .iter()
        .tuple_windows()
        .map(|(x, y, z)| x + y + z)
        .tuple_windows()
        .filter(|(x, y)| y > x)
        .count()
}
