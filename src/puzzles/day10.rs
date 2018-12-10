use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;
use regex::{Match, Regex};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[derive(Debug)]
struct Point {
    pub x: i64,
    pub y: i64,
    pub vel_x: i64,
    pub vel_y: i64,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums_re = Regex::new(r"(-?\d+)").unwrap();
        let nums: Vec<Match> = nums_re.find_iter(s).collect();

        Ok(Point {
            x: nums[0].as_str().parse().unwrap(),
            y: nums[1].as_str().parse().unwrap(),
            vel_x: nums[2].as_str().parse().unwrap(),
            vel_y: nums[3].as_str().parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Sky {
    points: Vec<Point>,
}

impl Sky {
    fn new(points: Vec<Point>) -> Sky {
        Sky { points }
    }

    fn step(&mut self) {
        for mut point in &mut self.points {
            point.x += point.vel_x;
            point.y += point.vel_y;
        }
    }

    fn no_wait_go_back(&mut self) {
        for mut point in &mut self.points {
            point.x -= point.vel_x;
            point.y -= point.vel_y;
        }
    }

    fn coord_bounds(&self) -> (i64, i64, i64, i64) {
        let mut min_x = std::i64::MAX;
        let mut min_y = std::i64::MAX;
        let mut max_x = std::i64::MIN;
        let mut max_y = std::i64::MIN;

        for point in &self.points {
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y > max_y {
                max_y = point.y;
            }

            if point.x < min_x {
                min_x = point.x;
            }
            if point.y < min_y {
                min_y = point.y;
            }
        }

        (min_x, max_x, min_y, max_y)
    }

    fn area(&self) -> i64 {
        let bounds = self.coord_bounds();
        (bounds.1 - bounds.0) * (bounds.3 - bounds.2)
    }

    fn print(&self) {
        let bounds = self.coord_bounds();
        let coords = self
            .points
            .iter()
            .map(|p| (p.x, p.y))
            .collect::<HashSet<(i64, i64)>>();

        let mut points_drawn = 0;

        for y in bounds.2..=bounds.3 {
            for x in bounds.0..=bounds.1 {
                if coords.contains(&(x, y)) {
                    print!("#");
                    points_drawn += 1;
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        println!("{} points drawn out of {}", points_drawn, coords.len());
    }
}

#[aoc(10)]
fn solve_1(input: Input) {
    let mut points = Vec::new();
    for point in input.parse_lines::<Point>() {
        points.push(point);
    }

    let mut sky = Sky::new(points);
    let mut last_area = std::i64::MAX;

    let mut iterations = 0;
    loop {
        sky.step();
        let new_area = sky.area();

        if new_area < last_area {
            last_area = new_area;
        } else {
            sky.no_wait_go_back();
            sky.print();
            println!("ended on area {} at iteration {}", new_area, iterations);
            break;
        }

        iterations += 1;
    }
}

#[aoc(10)]
fn solve_2(_input: Input) {
    println!("see iterations in part 1");
}
