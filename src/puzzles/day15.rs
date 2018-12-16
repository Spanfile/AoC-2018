use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[derive(Debug, PartialEq, Clone)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Debug, Clone)]
struct Unit {
    x: i32,
    y: i32,
    unit_type: UnitType,
    health: i32,
}

impl Unit {
    fn new(x: i32, y: i32, unit_type: UnitType) -> Unit {
        Unit {
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

    fn turn(&mut self, map: &Map, units: &Vec<&mut Unit>) -> bool {
        let possible_targets = self.find_possible_targets(map, units);
        if possible_targets.is_empty() {
            return false;
        }

        let squares_in_range = self.squares_in_range(map, &possible_targets);

        true
    }

    fn find_possible_targets<'a>(&self, map: &Map, units: &'a [&mut Unit]) -> Vec<&'a mut Unit> {
        units
            .into_iter()
            .filter(|u| u.unit_type == self.get_opponent_type())
            .collect::<Vec<&mut Unit>>()
    }

    fn get_opponent_type(&self) -> UnitType {
        match self.unit_type {
            UnitType::Elf => UnitType::Goblin,
            UnitType::Goblin => UnitType::Elf,
        }
    }

    fn squares_in_range(&self, map: &Map, targets: &[&mut Unit]) -> Vec<i32> {
        vec![0]
    }
}

#[derive(Debug)]
enum Tile {
    Floor,
    Wall,
}

impl Tile {
    fn print(&self) {
        match self {
            Tile::Floor => print!("."),
            Tile::Wall => print!("#"),
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    dimension: i32,
}

impl Map {
    fn print_with_units(&self, units: &[Unit]) {
        for y in 0..self.dimension {
            for x in 0..self.dimension {
                let unit_here = units.iter().find(|u| u.x == x && u.y == y);
                if unit_here.is_some() {
                    unit_here.unwrap().print();
                } else {
                    self.get_tile_at_coords(x, y).print();
                }
            }
            println!();
        }
    }

    fn get_tile_at_coords(&self, x: i32, y: i32) -> &Tile {
        &self.tiles[self.coords_to_index(x, y)]
    }

    fn coords_to_index(&self, x: i32, y: i32) -> usize {
        (x + y * self.dimension) as usize
    }
}

fn parse_map(input: Input) -> (Vec<Unit>, Map) {
    let mut tiles = Vec::new();
    let mut units = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().char_indices() {
            match c {
                '#' => tiles.push(Tile::Wall),
                '.' => tiles.push(Tile::Floor),
                'G' => {
                    tiles.push(Tile::Floor);
                    units.push(Unit::new(x as i32, y as i32, UnitType::Goblin));
                }
                'E' => {
                    tiles.push(Tile::Floor);
                    units.push(Unit::new(x as i32, y as i32, UnitType::Elf));
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
    )
}

#[aoc(15)]
fn solve_1(input: Input) {
    let (mut units, map) = parse_map(input);
    map.print_with_units(&units);

    for mut unit in &units {
        unit.turn(&map, &units);
    }
}

#[aoc(1)]
fn solve_2(input: Input) {}
