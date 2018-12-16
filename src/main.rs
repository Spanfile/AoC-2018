#![feature(duration_as_u128)]
#![feature(vec_remove_item)]

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
