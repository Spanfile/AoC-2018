use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

fn power_level(grid_serial: i32, x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = ((rack_id * y) + grid_serial) * rack_id;
    power_level = (power_level / 100) % 10;
    power_level -= 5;
    power_level
}

fn create_grid(grid_serial: i32) -> Vec<i32> {
    let mut grid = Vec::with_capacity(300 * 300);
    for y in 0..300 {
        for x in 0..300 {
            grid.push(power_level(grid_serial, x, y));
        }
    }

    grid
}

fn find_square(square: i32, grid: &Vec<i32>) -> (i32, i32, i32, i32) {
    let mut max = std::i32::MIN;
    let mut max_coords = (0, 0);
    let mut max_square = 1;

    for y in 1..=300 - square {
        for x in 1..=300 - square {
            let mut power = 0;
            for cell_y in 0..square {
                for cell_x in 0..square {
                    power += grid[((x + cell_x) + (y + cell_y) * 300) as usize];
                }
            }

            if power > max {
                max = power;
                max_coords = (x, y);
                max_square = square;
            }
        }
    }

    (max_coords.0, max_coords.1, max_square, max)
}

#[aoc(11)]
fn solve_1(input: Input) {
    let grid_serial = input.get().parse().unwrap();
    let grid = create_grid(grid_serial);
    let best_3by3 = find_square(3, &grid);

    println!("{},{}", best_3by3.0, best_3by3.1);
}

#[aoc(11)]
fn solve_2(input: Input) {
    let grid_serial = input.get().parse().unwrap();
    let grid = create_grid(grid_serial);

    let mut max = std::i32::MIN;
    let mut best: (i32, i32, i32, i32) = (0, 0, 0, 0);

    for square in 1..=300 {
        let best_power = find_square(square, &grid);

        if best_power.3 > max {
            max = best_power.3;
            best = best_power;
        }
    }

    println!("{},{},{}", best.0, best.1, best.2);
}
