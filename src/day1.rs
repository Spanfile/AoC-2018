use input;
use std::collections::HashSet;

pub fn solve_1() {
    let puzzle_input = input::get(1);
    let lines = puzzle_input.lines();

    let mut answer: i32 = 0;
    for s in lines {
        answer += s.parse::<i32>().unwrap();
    }

    println!("1: {}", answer);
}

pub fn solve_2() {
    let puzzle_input = input::get(1);
    let lines: Vec<&str> = puzzle_input.lines().collect();

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
