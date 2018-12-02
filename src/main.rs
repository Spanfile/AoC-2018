#![feature(duration_as_u128)]

extern crate aoc_derive;
extern crate counter;
extern crate reqwest;

mod input;
mod puzzles;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        puzzles::solve_latest();
    } else {
        let day = args[1].parse::<i32>().expect("couldn't parse input");
        puzzles::solve_day(day);
    }
}
