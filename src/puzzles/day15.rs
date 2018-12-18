use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[derive(Debug, PartialEq, Clone)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Floor,
    Wall,
}

#[derive(Debug, Clone)]
struct Unit {
    id: i32,
    x: i32,
    y: i32,
    unit_type: UnitType,
    health: i32,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    dimension: i32,
}

impl Unit {
    fn new(id: i32, x: i32, y: i32, unit_type: UnitType) -> Unit {
        Unit {
            id,
            x,
            y,
            unit_type,
            health: 200,
        }
    }

    fn print(&self) {
        print!(
            "{}",
            match self.unit_type {
                UnitType::Elf => "E",
                UnitType::Goblin => "G",
            }
        );
    }

    fn turn(&mut self, map: &Map, units: &HashMap<i32, RefCell<Unit>>) -> bool {
        let possible_targets = self.find_possible_targets(units);
        if possible_targets.is_empty() {
            return false;
        }

        let squares_in_range = self.squares_in_range(map, &possible_targets, units);
        let self_index = map.coords_to_index(self.x, self.y);
        let mut closest = (std::f64::MAX, std::usize::MAX);

        for (index, target) in &squares_in_range {
            if self_index == *index {
                self.attack(*target);
                return true;
            } else {
                let distance = map.distance_from_to(self_index, *index);
                if distance < closest.0 {
                    closest = (distance, *index);
                }
            }
        }

        self.move_towards(closest.1);

        true
    }

    fn attack(&self, target: i32) {}

    fn move_towards(&self, point_index: usize) {}

    fn find_possible_targets(&self, units: &HashMap<i32, RefCell<Unit>>) -> Vec<i32> {
        units
            .iter()
            .filter_map(|(id, unit)| {
                if *id != self.id && unit.borrow().unit_type == self.get_opponent_type() {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_opponent_type(&self) -> UnitType {
        match self.unit_type {
            UnitType::Elf => UnitType::Goblin,
            UnitType::Goblin => UnitType::Elf,
        }
    }

    fn squares_in_range(
        &self,
        map: &Map,
        target_units: &[i32],
        units: &HashMap<i32, RefCell<Unit>>,
    ) -> Vec<(usize, i32)> {
        let mut possible_squares = Vec::new();

        for target_id in target_units {
            let target_unit = units[&target_id].borrow();

            for x_step in (-1..1).step_by(2) {
                let check_x = target_unit.x + x_step;
                let (index, tile) = map.get_tile_at_coords(check_x, self.y);
                if *tile == Tile::Floor {
                    let unit_blocking = units.iter().find(|(id, unit_cell)| {
                        if *id == target_id || **id == self.id {
                            return false;
                        }
                        let unit = unit_cell.borrow();
                        map.coords_to_index(unit.x, unit.y) == index
                    });

                    if unit_blocking.is_none() {
                        possible_squares.push((index, *target_id));
                    }
                }
            }

            for y_step in (-1..1).step_by(2) {
                let check_y = target_unit.y + y_step;
                let (index, tile) = map.get_tile_at_coords(self.x, check_y);
                if *tile == Tile::Floor {
                    possible_squares.push((index, *target_id));
                }
            }
        }

        possible_squares
    }
}

impl Tile {
    fn print(&self) {
        match self {
            Tile::Floor => print!("."),
            Tile::Wall => print!("#"),
        }
    }
}

impl Map {
    fn print_with_units(&self, units: &HashMap<i32, RefCell<Unit>>) {
        for y in 0..self.dimension {
            for x in 0..self.dimension {
                let unit_here = units.values().find(|u| {
                    let u = u.borrow();
                    u.x == x && u.y == y
                });
                if unit_here.is_some() {
                    unit_here.unwrap().borrow().print();
                } else {
                    self.get_tile_at_coords(x, y).1.print();
                }
            }
            println!();
        }
    }

    fn get_tile_at_coords(&self, x: i32, y: i32) -> (usize, &Tile) {
        let index = self.coords_to_index(x, y);
        (index, &self.tiles[self.coords_to_index(x, y)])
    }

    fn coords_to_index(&self, x: i32, y: i32) -> usize {
        (x + y * self.dimension) as usize
    }

    fn index_to_coords(&self, index: usize) -> (i32, i32) {
        (
            index as i32 % self.dimension,
            (index as i32 / self.dimension) % self.dimension,
        )
    }

    fn distance_from_to(&self, from: usize, to: usize) -> f64 {
        let from = self.index_to_coords(from);
        let to = self.index_to_coords(to);

        f64::from((to.0 - from.0).pow(2) + (to.1 - from.1).pow(2)).sqrt()
    }
}

fn parse_input(input: Input) -> (HashMap<i32, RefCell<Unit>>, Map, i32) {
    let mut tiles = Vec::new();
    let mut units = HashMap::new();
    let mut unit_id = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().char_indices() {
            match c {
                '#' => tiles.push(Tile::Wall),
                '.' => tiles.push(Tile::Floor),
                'G' => {
                    tiles.push(Tile::Floor);
                    units.insert(
                        unit_id,
                        RefCell::new(Unit::new(unit_id, x as i32, y as i32, UnitType::Goblin)),
                    );
                    unit_id += 1;
                }
                'E' => {
                    tiles.push(Tile::Floor);
                    units.insert(
                        unit_id,
                        RefCell::new(Unit::new(unit_id, x as i32, y as i32, UnitType::Elf)),
                    );
                    unit_id += 1;
                }
                _ => println!("unknown character {} when parsing input", c),
            }
        }
    }

    (
        units,
        Map {
            tiles,
            dimension: 32,
        },
        unit_id - 1,
    )
}

#[aoc(15)]
fn solve_1(input: Input) {
    let (units, map, unit_max_id) = parse_input(input);

    map.print_with_units(&units);

    let mut result = false;
    let mut turn_counter = 0;
    loop {
        turn_counter += 1;
        println!("Turn {}", turn_counter);

        for id in 0..unit_max_id {
            let mut unit = units[&id].borrow_mut();
            result |= unit.turn(&map, &units);
        }

        // wtf rust, where is my do-while?
        if !result {
            break;
        }
    }
}

#[aoc(1)]
fn solve_2(input: Input) {}
