use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

struct Claim {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args: Vec<&str> = s.split_whitespace().collect();

        let id = args[0][1..].parse().unwrap();

        let xy: Vec<&str> = args[2].split(',').collect();
        let x = xy[0].parse().unwrap();
        let y = xy[1].trim_end_matches(':').parse().unwrap();

        let wh: Vec<&str> = args[3].split('x').collect();
        let width = wh[0].parse().unwrap();
        let height = wh[1].parse().unwrap();

        // println!("#{} @ {},{}: {}x{}", id, x, y, w, h);

        Ok(Claim {
            id,
            x,
            y,
            width,
            height,
        })
    }
}

impl Claim {
    fn plane_coords(&self) -> Vec<(i32, i32)> {
        let mut indices: Vec<(i32, i32)> = Vec::new();

        for y in self.y..self.y + self.height {
            for x in self.x..self.x + self.width {
                indices.push((x, y));
            }
        }

        indices
    }

    fn collides_with(&self, other: &Claim) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[aoc(3)]
fn solve_1(input: Input) {
    let mut fabric: HashMap<(i32, i32), i32> = HashMap::with_capacity(1301);
    let mut overlaps = 0;

    for claim in input.parse_lines::<Claim>() {
        for i in claim.plane_coords() {
            let entry = fabric.entry(i).or_insert(0);
            *entry += 1;

            if *entry == 2 {
                overlaps += 1;
            }
        }
    }

    println!("{}", overlaps);
}

#[aoc(3)]
fn solve_2(input: Input) {
    let mut fabric: HashMap<i32, Claim> = HashMap::new();
    let mut ids = HashSet::new();
    let mut to_delete = HashSet::new();

    for claim in input.parse_lines::<Claim>() {
        for (id, other) in &fabric {
            if claim.collides_with(other) {
                to_delete.insert(claim.id);
                to_delete.insert(*id);
            }
        }

        ids.insert(claim.id);
        fabric.insert(claim.id, claim);
    }

    let diff = ids.difference(&to_delete);
    println!("{:?}", diff);
}
