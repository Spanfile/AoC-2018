use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[aoc(9)]
fn solve_1(_input: Input) {
    let player_count = 411;
    let last_marble_value = 72059;

    let mut marbles = vec![0];
    let mut current_marble_index = 0;
    let mut current_marble_value = 0;
    let mut players: HashMap<i32, i32> = HashMap::new();
    let mut current_player = 0;

    while current_marble_value <= last_marble_value {
        let mut new_marble_index = current_marble_index + 2;
        current_marble_value += 1;

        if current_marble_value % 23 == 0 {
            let score = players.entry(current_player).or_insert(0);
            *score += current_marble_value;

            let mut additional_marble = current_marble_index as i32 - 7;

            while additional_marble < 0 {
                additional_marble += marbles.len() as i32;
            }

            while additional_marble >= marbles.len() as i32 {
                additional_marble -= marbles.len() as i32;
            }

            *score += marbles.remove(additional_marble as usize);
            new_marble_index = additional_marble as usize;
        } else {
            while new_marble_index >= marbles.len() {
                new_marble_index -= marbles.len();
            }

            if new_marble_index == marbles.len() - 1 {
                marbles.push(current_marble_value);
            } else {
                marbles.insert(new_marble_index, current_marble_value);
            }
        }

        current_marble_index = new_marble_index;
        current_player = (current_player + 1) % player_count;
    }

    let highest = players.values().max().unwrap();
    println!("{}", highest);
}

#[aoc(9)]
fn solve_2(_input: Input) {
    let player_count = 411;
    let last_marble_value = 7_205_900;

    let mut players: HashMap<i32, i64> = HashMap::new();
    let mut circle: VecDeque<i64> = VecDeque::new();
    circle.push_back(0);

    for value in 1..=last_marble_value {
        if value % 23 == 0 {
            for _ in 0..7 {
                let back = circle.pop_back().unwrap();
                circle.push_front(back);
            }

            let score = players.entry(value % player_count).or_insert(0);
            *score += i64::from(value) + circle.pop_back().unwrap();

            let front = circle.pop_front().unwrap();
            circle.push_back(front);
        } else {
            let front = circle.pop_front().unwrap();
            circle.push_back(front);
            circle.push_back(i64::from(value));
        }
    }

    let highest = players.values().max().unwrap();
    println!("{}", highest);
}
