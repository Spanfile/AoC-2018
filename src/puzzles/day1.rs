use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;
use std::collections::HashSet;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[aoc(1)]
fn solve_1(input: Input) {
    let mut answer: i32 = 0;
    for s in input.lines() {
        answer += s.parse::<i32>().unwrap();
    }

    println!("{}", answer);
}

#[aoc(1)]
fn solve_2(input: Input) {
    let lines: Vec<&str> = input.lines().collect();

    let mut freqs = HashSet::new();
    let mut cur = 0;
    loop {
        for s in &lines {
            cur += s.parse::<i32>().unwrap();
            if freqs.contains(&cur) {
                println!("{}", cur);
                return;
            }
            freqs.insert(cur);
        }
    }
}
