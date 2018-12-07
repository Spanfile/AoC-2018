use super::runner;
use crate::input;
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
                &new_chars.push(*c);
                break;
            }

            let check_next = match c.is_ascii_uppercase() {
                true => c.to_ascii_lowercase(),
                false => c.to_ascii_uppercase(),
            };

            if chars[i + 1] == check_next {
                // println!("{} == {}", c, chars[i + 1]);
                drop_next = true;
                continue;
            }

            &new_chars.push(*c);
        }

        if chars.len() > new_chars.len() {
            // println!(
            //     "{} characters dropped ({} vs {}) on iteration {}",
            //     chars.len() - new_chars.len(),
            //     chars.len(),
            //     new_chars.len(),
            //     iter
            // );
            chars = new_chars.clone();
            new_chars.clear();
        } else {
            // println!("no changes on iteration {}", iter);
            break;
        }
    }

    chars.len() as i32
}

#[aoc(5)]
fn solve_1(input: String) {
    println!("{}", react_polymer(input));
}

#[aoc(5)]
fn solve_2(input: String) {
    let alphabet = "abcdefghijklmopqrstuvwxyz";
    let mut shortest = 1000000000;

    for letter in alphabet.chars() {
        let mut polymer = input.clone();
        polymer.retain(|c| c.to_ascii_lowercase() != letter);
        let reacted = react_polymer(polymer);
        if reacted < shortest {
            shortest = reacted;
        }
    }

    println!("{}", shortest);
}
