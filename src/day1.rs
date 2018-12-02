#[macro_use]
use aoc_derive::aoc;
use std::collections::HashSet;

#[aoc(1)]
pub fn solve_1(input: String) {
    let lines = input.lines();

    let mut answer: i32 = 0;
    for s in lines {
        answer += s.parse::<i32>().unwrap();
    }

    println!("1: {}", answer);
}

pub fn solve_2(input: String) {
    let lines: Vec<&str> = input.lines().collect();

    let mut freqs = HashSet::new();
    let mut cur = 0;
    loop {
        for s in &lines {
            cur += s.parse::<i32>().unwrap();
            if freqs.contains(&cur) {
                println!("2: {}", cur);
                return;
            }
            freqs.insert(cur);
        }
    }
}
