use super::runner;
use aoc_derive::aoc;
use input;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
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

    fn plane_coords_set(&self) -> HashSet<(i32, i32)> {
        let mut indices = HashSet::new();

        for y in self.y..self.y + self.h {
            for x in self.x..self.x + self.w {
                indices.insert((x, y));
            }
        }

        indices
    }
}

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[aoc(3)]
fn solve_1(input: String) {
    let lines = input.lines();

    let mut fabric = HashSet::new();
    let mut counted = HashSet::new();
    let mut overlaps = 0;

    for line in lines {
        let claim = line.parse::<Claim>().unwrap();
        for i in claim.plane_coords() {
            if fabric.contains(&i) && !counted.contains(&i) {
                overlaps += 1;
                counted.insert(i);
                continue;
            }

            fabric.insert(i);
        }
    }

    println!("{}", overlaps);
}

#[aoc(3)]
fn solve_2(input: String) {
    let lines = input.lines();

    let mut fabric: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();
    let mut to_delete = HashSet::new();

    for line in lines {
        let claim = line.parse::<Claim>().unwrap();
        fabric.insert(claim.id, claim.plane_coords_set());
    }

    for (id1, coords1) in &fabric {
        for (id2, coords2) in &fabric {
            if id1 == id2 {
                continue;
            }

            if !coords1.is_disjoint(coords2) {
                to_delete.insert(*id1);
                to_delete.insert(*id2);
            }
        }
    }

    let keys: HashSet<i32> = HashSet::from_iter(fabric.keys().cloned());
    let diff = keys.difference(&to_delete);
    println!("{:?}", diff);
}
