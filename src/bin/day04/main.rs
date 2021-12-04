#![feature(drain_filter)]

use arrayvec::ArrayVec;
use colored::Colorize;
use lazy_static::lazy_static;
use std::fmt::Display;

#[derive(Clone, Copy)]
struct Entry {
    number: u8,
    marked: bool,
}

#[derive(Clone)]
struct Board(ArrayVec<ArrayVec<Entry, 5>, 5>);

impl Board {
    fn call(&mut self, number: u8) {
        for row in &mut self.0 {
            for entry in row {
                if entry.number == number {
                    entry.marked = true;
                }
            }
        }
    }

    fn is_winning(&self) -> bool {
        // horizontal
        for row in &self.0 {
            if row.iter().all(|entry| entry.marked) {
                return true;
            }
        }
        // vertical
        for i in 0..5 {
            if self.0.iter().all(|row| row[i].marked) {
                return true;
            }
        }
        // DIAGONALS DON'T COUNT??? WHATTA HELL
        // // diagonal
        // if (0..5).all(|i| self.0[i][i].marked) {
        //     return true;
        // }
        // // antidiagonal
        // if (0..5).all(|i| self.0[i][4 - i].marked) {
        //     return true;
        // }
        false
    }

    fn score(&self, last_call: u8) -> usize {
        let sum = self
            .0
            .iter()
            .flat_map(|row| row.iter())
            .filter_map(|entry| {
                if !entry.marked {
                    Some(entry.number as usize)
                } else {
                    None
                }
            })
            .sum::<usize>();
        sum * last_call as usize
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            writeln!(f)?;
            for entry in row {
                if entry.marked {
                    write!(
                        f,
                        "{} ",
                        format!("{:2}", entry.number).black().on_bright_white()
                    )?;
                } else {
                    write!(f, "{:2} ", entry.number)?;
                }
            }
        }
        Ok(())
    }
}

static INPUT: &'static str = include_str!("input.txt");

lazy_static! {
    static ref CALLS: Vec<u8> = INPUT
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    static ref BOARDS: Vec<Board> = INPUT
        .split("\n\n")
        .skip(1)
        .map(|b| Board(
            b.lines()
                .map(|l| {
                    l.split_whitespace()
                        .map(|n| Entry {
                            number: n.parse().unwrap(),
                            marked: false,
                        })
                        .collect()
                })
                .collect()
        ))
        .collect();
}

fn main() {
    println!("*  {}", part1());
    println!("** {}", part2());
}

fn part1() -> impl Display {
    let mut boards = BOARDS.clone();
    for &call in &*CALLS {
        for board in &mut boards {
            board.call(call);
        }
        for board in &boards {
            if board.is_winning() {
                // println!("{}", board);
                return board.score(call);
            }
        }
    }
    panic!("no winner found")
}

fn part2() -> impl Display {
    let mut boards = BOARDS.clone();
    for &call in &*CALLS {
        for board in &mut boards {
            board.call(call);
        }
        if boards.len() == 1 && boards[0].is_winning() {
            // println!("{}", boards[0]);
            return boards[0].score(call);
        }
        boards.drain_filter(|board| board.is_winning());
    }
    panic!("no last board found")
}
