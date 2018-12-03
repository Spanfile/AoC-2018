use super::runner;
use aoc_derive::aoc;
use input;
use std::collections::HashMap;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

struct Claim {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args: Vec<&str> = s.split_whitespace().into_iter().collect();

        let id: i32 = args[0][1..].parse::<i32>().unwrap();

        let xy: Vec<&str> = args[2].split(",").into_iter().collect();
        let x = xy[0].parse::<i32>().unwrap();
        let y = xy[1].trim_right_matches(':').parse::<i32>().unwrap();

        let wh: Vec<&str> = args[3].split("x").into_iter().collect();
        let w = wh[0].parse::<i32>().unwrap();
        let h = wh[1].parse::<i32>().unwrap();

        // println!("#{} @ {},{}: {}x{}", id, x, y, w, h);

        Ok(Claim { id, x, y, w, h })
    }
}

impl Claim {
    fn plane_coords(&self) -> Vec<(i32, i32)> {
        let mut indices: Vec<(i32, i32)> = Vec::new();

        for y in self.y..self.y + self.h {
            for x in self.x..self.x + self.w {
                indices.push((x, y));
            }
        }

        indices
    }

    fn collides_with(&self, other: &Claim) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[aoc(3)]
fn solve_1(input: String) {
    let lines = input.lines();

    let mut fabric: HashMap<(i32, i32), i32> = HashMap::with_capacity(1301);
    let mut overlaps = 0;

    for line in lines {
        let claim = line.parse::<Claim>().unwrap();
        for i in claim.plane_coords() {
            let mut entry = fabric.entry(i).or_insert(0);
            *entry += 1;

            if *entry == 2 {
                overlaps += 1;
            }
        }
    }

    println!("{}", overlaps);
}

#[aoc(3)]
fn solve_2(input: String) {
    let lines = input.lines();

    let mut fabric: HashMap<i32, Claim> = HashMap::new();
    let mut ids = HashSet::new();
    let mut to_delete = HashSet::new();

    for line in lines {
        let claim = line.parse::<Claim>().unwrap();

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
