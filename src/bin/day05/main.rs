use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp::Ordering, collections::BTreeMap, fmt::Display};

#[derive(Clone, Copy)]
struct Line {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }

    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    fn is_diagonal(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }
}

struct LineIter {
    line: Line,
    xdir: Ordering,
    ydir: Ordering,
    current: (u16, u16),
    done: bool,
}

impl Iterator for LineIter {
    type Item = (u16, u16);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let next = self.current;
            if self.current == (self.line.x2, self.line.y2) {
                self.done = true;
            } else {
                match self.xdir {
                    Ordering::Less => self.current.0 += 1,
                    Ordering::Greater => self.current.0 -= 1,
                    _ => {}
                }
                match self.ydir {
                    Ordering::Less => self.current.1 += 1,
                    Ordering::Greater => self.current.1 -= 1,
                    _ => {}
                }
            }
            Some(next)
        }
    }
}

impl IntoIterator for &Line {
    type Item = (u16, u16);

    type IntoIter = LineIter;

    fn into_iter(self) -> Self::IntoIter {
        LineIter {
            line: self.clone(),
            xdir: self.x1.cmp(&self.x2),
            ydir: self.y1.cmp(&self.y2),
            current: (self.x1, self.y1),
            done: false,
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} -> {},{} [{}]",
            self.x1,
            self.y1,
            self.x2,
            self.y2,
            self.into_iter()
                .format_with(", ", |(x, y), f| f(&format_args!("{},{}", x, y)))
        )
    }
}

lazy_static! {
    static ref LINES: Vec<Line> = {
        let regex = Regex::new(r"\d+").unwrap();
        include_str!("input.txt")
            .lines()
            .map(|l| {
                let (x1, y1, x2, y2) = regex
                    .find_iter(l)
                    .map(|n| n.as_str().parse().unwrap())
                    .next_tuple()
                    .unwrap();
                Line { x1, y1, x2, y2 }
            })
            .collect()
    };
}

fn main() {
    println!("*  {}", part1());
    println!("** {}", part2());
}

fn part1() -> impl Display {
    let mut vents: BTreeMap<_, usize> = BTreeMap::new();
    for line in &*LINES {
        if line.is_diagonal() {
            continue;
        }
        for pos in line {
            *vents.entry(pos).or_default() += 1;
        }
    }
    vents.values().filter(|&&n| n >= 2).count()
}

fn part2() -> impl Display {
    let mut vents: BTreeMap<_, usize> = BTreeMap::new();
    for line in &*LINES {
        for pos in line {
            *vents.entry(pos).or_default() += 1;
        }
    }
    vents.values().filter(|&&n| n >= 2).count()
}
