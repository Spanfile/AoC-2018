use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

fn digits(value: i32) -> Vec<i32> {
    value
        .to_string()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect::<Vec<i32>>()
}

#[aoc(14)]
fn solve_1(_input: Input) {
    let end_at = 286_051 + 10;
    let mut board = vec![3, 7];
    let mut elf1_index: i32 = 0;
    let mut elf2_index: i32 = 1;
    let mut recipes_made = 0;

    while recipes_made <= end_at {
        let new_recipes = digits(board[elf1_index as usize] + board[elf2_index as usize]);
        recipes_made += new_recipes.len();
        board.extend(new_recipes);
        elf1_index = (elf1_index + board[elf1_index as usize] + 1) % board.len() as i32;
        elf2_index = (elf2_index + board[elf2_index as usize] + 1) % board.len() as i32;
    }

    println!(
        "{:?}",
        board[end_at - 10..end_at]
            .iter()
            .map(|i| i.to_string())
            .collect::<String>()
    );
}

#[aoc(14)]
fn solve_2(_input: Input) {
    let end_at = digits(286_051);
    let mut board = vec![3, 7];
    let mut elf1_index: i32 = 0;
    let mut elf2_index: i32 = 1;

    loop {
        let new_recipes = digits(board[elf1_index as usize] + board[elf2_index as usize]);
        let new_recipe_len = new_recipes.len();
        board.extend(new_recipes);
        elf1_index = (elf1_index + board[elf1_index as usize] + 1) % board.len() as i32;
        elf2_index = (elf2_index + board[elf2_index as usize] + 1) % board.len() as i32;

        if board.len() < 6 {
            continue;
        }

        if board.len() >= 6 && board[board.len() - 6..].to_vec() == end_at {
            println!("{}", board.len() - 6);
            break;
        }

        if new_recipe_len == 2 {
            if board.len() >= 7 && board[board.len() - 7..board.len() - 1].to_vec() == end_at {
                println!("{}", board.len() - 7);
                break;
            }
        }
    }
}
