use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

fn react_polymer(polymer: String) -> i32 {
    let mut chars = polymer.chars().collect::<Vec<char>>();

    let mut new_chars = Vec::new();
    let mut drop_next = false;

    loop {
        for (i, c) in chars[..chars.len()].iter().enumerate() {
            if drop_next {
                drop_next = false;
                continue;
            }

            if i == chars.len() - 1 {
                new_chars.push(*c);
                break;
            }

            let check_next = if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            };

            if chars[i + 1] == check_next {
                drop_next = true;
                continue;
            }

            new_chars.push(*c);
        }

        if chars.len() > new_chars.len() {
            chars = new_chars.clone();
            new_chars.clear();
        } else {
            break;
        }
    }

    chars.len() as i32
}

#[aoc(5)]
fn solve_1(input: Input) {
    println!("{}", react_polymer(input.get()));
}

#[aoc(5)]
fn solve_2(input: Input) {
    let alphabet = "abcdefghijklmopqrstuvwxyz";
    let mut shortest = 1_000_000;

    for letter in alphabet.chars() {
        let mut polymer = input.clone().get();
        polymer.retain(|c| c.to_ascii_lowercase() != letter);
        let reacted = react_polymer(polymer.to_string());
        if reacted < shortest {
            shortest = reacted;
        }
    }

    println!("{}", shortest);
}
