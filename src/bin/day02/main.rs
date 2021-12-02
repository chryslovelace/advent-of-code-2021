use itertools::Itertools;
use lazy_static::lazy_static;
use std::fmt::Display;

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

lazy_static! {
    static ref INPUT: Vec<Command> = include_str!("input.txt")
        .lines()
        .map(|l| {
            let (dir, n) = l.split(' ').next_tuple().unwrap();
            let n = n.parse().unwrap();
            match dir {
                "forward" => Command::Forward(n),
                "down" => Command::Down(n),
                "up" => Command::Up(n),
                _ => panic!("unrecognized command"),
            }
        })
        .collect();
}

fn main() {
    println!("*  {}", part1());
    println!("** {}", part2());
}

fn part1() -> impl Display {
    let (mut pos, mut depth) = (0, 0);
    for command in &*INPUT {
        match command {
            Command::Forward(n) => pos += n,
            Command::Down(n) => depth += n,
            Command::Up(n) => depth -= n,
        }
    }
    pos * depth
}

fn part2() -> impl Display {
    let (mut pos, mut depth, mut aim) = (0, 0, 0);
    for command in &*INPUT {
        match command {
            Command::Forward(n) => {
                pos += n;
                depth += aim * n;
            }
            Command::Down(n) => aim += n,
            Command::Up(n) => aim -= n,
        }
    }
    pos * depth
}
