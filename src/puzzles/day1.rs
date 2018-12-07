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
    for num in input.parse_lines::<i32>() {
        answer += num;
    }

    println!("{}", answer);
}

#[aoc(1)]
fn solve_2(input: Input) {
    let mut freqs = HashSet::new();
    let mut cur = 0;
    loop {
        for num in input.clone().parse_lines::<i32>() {
            cur += num;
            if freqs.contains(&cur) {
                println!("{}", cur);
                return;
            }
            freqs.insert(cur);
        }
    }
}
