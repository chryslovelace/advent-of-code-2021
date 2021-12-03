#![feature(drain_filter)]

use bitvec::prelude::*;
use lazy_static::lazy_static;
use std::fmt::Display;

lazy_static! {
    static ref INPUT: Vec<BitVec<Msb0>> = {
        include_str!("input.txt")
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '0' => false,
                        '1' => true,
                        _ => panic!("unrecognized character in input"),
                    })
                    .collect()
            })
            .collect()
    };
}

fn main() {
    println!("*  {}", part1());
    println!("** {}", part2());
}

fn part1() -> impl Display {
    let width = INPUT[0].len();
    let mut count = vec![0; width];
    for item in &*INPUT {
        for i in 0..width {
            if item[i] {
                count[i] += 1;
            }
        }
    }
    let (mut gamma, mut epsilon) = (0, 0);
    for i in 0..width {
        if count[i] < INPUT.len() / 2 {
            gamma += 1 << (width - i - 1);
        } else {
            epsilon += 1 << (width - i - 1);
        }
    }
    gamma * epsilon
}

fn part2() -> impl Display {
    let width = INPUT[0].len();
    let oxy: usize = {
        let mut candidates = INPUT.clone();
        for i in 0..width {
            let count: usize = candidates.iter().map(|v| v[i] as usize).sum();
            if count >= candidates.len() / 2 {
                // 1 is most common/tied
                candidates.drain_filter(|v| !v[i]);
            } else {
                // 0 is most common
                candidates.drain_filter(|v| v[i]);
            }
            if candidates.len() == 1 {
                break;
            }
        }
        candidates[0].load()
    };
    let co2: usize = {
        let mut candidates = INPUT.clone();
        for i in 0..width {
            let count: usize = candidates.iter().map(|v| v[i] as usize).sum();
            if count < candidates.len() / 2 {
                // 1 is least common
                candidates.drain_filter(|v| !v[i]);
            } else {
                // 0 is least common/tied
                candidates.drain_filter(|v| v[i]);
            }
            if candidates.len() == 1 {
                break;
            }
        }
        candidates[0].load()
    };
    oxy * co2
}
