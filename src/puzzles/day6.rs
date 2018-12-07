use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn manhattan(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = s.split(", ").collect::<Vec<_>>();
        let x = args[0].parse().unwrap();
        let y = args[1].parse().unwrap();

        Ok(Point { x, y })
    }
}

impl Default for Point {
    fn default() -> Point {
        Point { x: 0, y: 0 }
    }
}

#[aoc(6)]
fn solve_1(input: Input) {
    let mut dists: HashMap<Point, i32> = HashMap::new();
    for s in input.lines() {
        let point = s.parse().unwrap();
        dists.insert(point, 0);
    }

    let min_x = dists.keys().map(|p| p.x).min().unwrap();
    let min_y = dists.keys().map(|p| p.y).min().unwrap();
    let max_x = dists.keys().map(|p| p.x).max().unwrap();
    let max_y = dists.keys().map(|p| p.y).max().unwrap();

    for y in min_y..max_y {
        'outer: for x in min_x..max_x {
            let point = Point { x, y };

            let mut min = 1_000_000_000;
            let mut min_point = Default::default();
            for p in dists.keys() {
                let dist = point.manhattan(*p);
                if dist == min {
                    continue 'outer;
                } else if dist < min {
                    min = dist;
                    min_point = *p;
                }
            }

            let entry = dists.entry(min_point).or_insert(0);
            *entry += 1;
        }
    }

    let max_area = dists.values().max().unwrap();
    println!("{}", max_area);
}

#[aoc(6)]
fn solve_2(input: Input) {
    let mut points: Vec<Point> = Vec::new();
    for s in input.lines() {
        points.push(s.parse().unwrap());
    }

    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let mut dists: HashMap<Point, i32> = HashMap::new();
    for y in min_y..max_y {
        for x in min_x..max_x {
            let point = Point { x, y };

            let mut total = 0;
            for p in &points {
                total += point.manhattan(*p);
            }

            let entry = dists.entry(point).or_insert(0);
            *entry += total;
        }
    }

    let area = dists.values().filter(|&d| *d < 10000).count();
    println!("{}", area);
}
