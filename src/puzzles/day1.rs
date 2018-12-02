use aoc_derive::aoc;
use std::collections::HashSet;
use input;
use super::runner;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[aoc(1)]
fn solve_1(input: String) {
    let lines = input.lines();

    let mut answer: i32 = 0;
    for s in lines {
        answer += s.parse::<i32>().unwrap();
    }

    println!("{}", answer);
}

#[aoc(1)]
fn solve_2(input: String) {
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
