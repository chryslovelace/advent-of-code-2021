#![feature(int_abs_diff)]

use itertools::Itertools;
use lazy_static::lazy_static;
use std::fmt::Display;

lazy_static! {
    static ref CRABS: Vec<u16> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
}

fn main() {
    println!("*  {}", part1());
    println!("** {}", part2());
}

fn part1() -> impl Display {
    let (&min, &max) = CRABS.iter().minmax().into_option().unwrap();
    (min..=max)
        .map(|position| {
            CRABS
                .iter()
                .map(|crab| crab.abs_diff(position) as u32)
                .sum::<u32>()
        })
        .min()
        .unwrap()
}

fn part2() -> impl Display {
    fn triangle(n: u32) -> u32 {
        n * (n + 1) / 2
    }

    let (&min, &max) = CRABS.iter().minmax().into_option().unwrap();
    (min..=max)
        .map(|position| {
            CRABS
                .iter()
                .map(|crab| triangle(crab.abs_diff(position) as u32))
                .sum::<u32>()
        })
        .min()
        .unwrap()
}
