use lazy_static::lazy_static;
use std::fmt::Display;
use std::iter::repeat;

#[derive(Clone, Copy)]
struct Lanternfish {
    timer: u8,
}

impl Lanternfish {
    fn step(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = 6;
            true
        } else {
            self.timer -= 1;
            false
        }
    }
}

lazy_static! {
    static ref INITIAL_STATE: Vec<Lanternfish> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|n| Lanternfish {
            timer: n.parse().unwrap()
        })
        .collect();
}

fn main() {
    println!("*  {}", part1());
    println!("** {}", part2());
}

fn part1() -> impl Display {
    let mut state = INITIAL_STATE.clone();
    for _ in 0..80 {
        let mut new_fish = 0;
        for fish in &mut state {
            if fish.step() {
                new_fish += 1;
            }
        }
        state.extend(repeat(Lanternfish { timer: 8 }).take(new_fish));
    }
    state.len()
}

fn part2() -> impl Display {
    let mut cohorts = [0usize; 9];
    for fish in &*INITIAL_STATE {
        cohorts[fish.timer as usize] += 1;
    }
    for _ in 0..256 {
        cohorts = [
            cohorts[1],
            cohorts[2],
            cohorts[3],
            cohorts[4],
            cohorts[5],
            cohorts[6],
            cohorts[7] + cohorts[0],
            cohorts[8],
            cohorts[0],
        ];
    }
    cohorts.into_iter().sum::<usize>()
}
