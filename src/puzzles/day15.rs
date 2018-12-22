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

#[derive(Debug)]
enum Occupation {
    Unit(usize, i32),
    Free(usize),
    Wall(usize),
}

#[derive(Debug, Clone)]
struct Unit {
    id: i32,
    x: i32,
    y: i32,
    unit_type: UnitType,
    health: i32,
    attack_power: i32,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    dimension: i32,
}

#[derive(Debug)]
struct Cave {
    map: Map,
    units: HashMap<i32, RefCell<Unit>>,
    unit_max_id: i32,
}

impl Unit {
    fn new(id: i32, x: i32, y: i32, unit_type: UnitType) -> Unit {
        Unit {
            id,
            x,
            y,
            unit_type,
            health: 200,
            attack_power: 3,
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

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn turn(&mut self, cave: &Cave) -> bool {
        if !self.is_alive() {
            println!("{:?} {} is dead", self.unit_type, self.id);
            return false;
        }

        let possible_targets = self.find_possible_targets(cave);
        if possible_targets.is_empty() {
            return false;
        }

        let adjacent_enemies = self.get_adjacent_enemies(cave);
        if !adjacent_enemies.is_empty() {
            self.attack(cave, adjacent_enemies);
            return true;
        }

        let squares_in_range = self.squares_in_range(cave, &possible_targets);
        let self_index = cave.map.coords_to_index(self.x, self.y);
        let mut closest = (std::f64::MAX, 0);

        for (index, _) in &squares_in_range {
            let distance = cave.map.distance_from_to(self_index, *index);
            if distance < closest.0 {
                closest = (distance, *index);
            }
        }

        self.move_towards(cave, closest.1);

        true
    }

    fn attack(&self, cave: &Cave, enemies: Vec<i32>) {
        let mut best = (std::i32::MAX, -1);

        for id in &enemies {
            let enemy = cave.units[id].borrow();
            if enemy.health < best.0 {
                best = (enemy.health, *id);
            }
        }

        let mut target = cave.units[&best.1].borrow_mut();
        target.health -= self.attack_power;

        println!(
            "{:?} {} at {},{} attacks {:?} {} at {},{} leaving them at {} health",
            self.unit_type,
            self.id,
            self.x,
            self.y,
            target.unit_type,
            target.id,
            target.x,
            target.y,
            target.health
        );
    }

    fn move_towards(&mut self, cave: &Cave, point_index: usize) {
        let adjacent = self.get_free_adjacent(cave);
        let mut best = (std::f64::MAX, 0);

        for adj in &adjacent {
            let dist = cave.map.distance_from_to(*adj, point_index);
            if dist < best.0 {
                best = (dist, *adj);
            }
        }

        self.move_to_index(cave, best.1 as i32);
    }

    fn move_to_index(&mut self, cave: &Cave, index: i32) {
        let new_x = index % cave.map.dimension;
        let new_y = (index / cave.map.dimension) % cave.map.dimension;
        println!(
            "{:?} {} moves from {},{} to {},{}",
            self.unit_type, self.id, self.x, self.y, new_x, new_y
        );
        self.x = new_x;
        self.y = new_y;
    }

    fn get_free_adjacent(&self, cave: &Cave) -> Vec<usize> {
        let mut adjacent = Vec::new();

        // this order is important; it's in reading-order of the tile indices
        if let Occupation::Free(index) = cave.check_occupation(self.x, self.y - 1) {
            adjacent.push(index);
        }

        if let Occupation::Free(index) = cave.check_occupation(self.x - 1, self.y) {
            adjacent.push(index);
        }

        if let Occupation::Free(index) = cave.check_occupation(self.x + 1, self.y) {
            adjacent.push(index);
        }

        if let Occupation::Free(index) = cave.check_occupation(self.x, self.y + 1) {
            adjacent.push(index);
        }

        adjacent
    }

    fn get_adjacent_enemies(&self, cave: &Cave) -> Vec<i32> {
        let mut enemies = Vec::new();

        if let Occupation::Unit(_, id) = cave.check_occupation(self.x, self.y - 1) {
            let enemy = cave.units[&id].borrow();
            if enemy.unit_type == self.get_opponent_type() && enemy.is_alive() {
                enemies.push(id);
            }
        }

        if let Occupation::Unit(_, id) = cave.check_occupation(self.x - 1, self.y) {
            let enemy = cave.units[&id].borrow();
            if enemy.unit_type == self.get_opponent_type() && enemy.is_alive() {
                enemies.push(id);
            }
        }

        if let Occupation::Unit(_, id) = cave.check_occupation(self.x + 1, self.y) {
            let enemy = cave.units[&id].borrow();
            if enemy.unit_type == self.get_opponent_type() && enemy.is_alive() {
                enemies.push(id);
            }
        }

        if let Occupation::Unit(_, id) = cave.check_occupation(self.x, self.y + 1) {
            let enemy = cave.units[&id].borrow();
            if enemy.unit_type == self.get_opponent_type() && enemy.is_alive() {
                enemies.push(id);
            }
        }

        enemies
    }

    fn find_possible_targets(&self, cave: &Cave) -> Vec<i32> {
        cave.units
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

    fn squares_in_range(&self, cave: &Cave, target_units: &[i32]) -> Vec<(usize, i32)> {
        let mut possible_squares = Vec::new();

        for target_id in target_units {
            let target_unit = cave.units[&target_id].borrow();

            for x_step in (-1..1).step_by(2) {
                let check_x = target_unit.x + x_step;
                if let Occupation::Free(index) = cave.check_occupation(check_x, self.y) {
                    possible_squares.push((index, *target_id));
                }
            }

            for y_step in (-1..1).step_by(2) {
                let check_y = target_unit.y + y_step;
                if let Occupation::Free(index) = cave.check_occupation(self.x, check_y) {
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

impl Cave {
    fn new(map: Map, units: HashMap<i32, RefCell<Unit>>, unit_max_id: i32) -> Cave {
        Cave {
            map,
            units,
            unit_max_id,
        }
    }

    fn print(&self) {
        self.map.print_with_units(&self.units);
    }

    fn turn(&self) -> bool {
        let mut id_indices = self
            .units
            .iter()
            .map(|(id, unit_cell)| {
                let unit = unit_cell.borrow();
                (*id, self.map.coords_to_index(unit.x, unit.y))
            })
            .collect::<Vec<(i32, usize)>>();
        id_indices.sort_by(|a, b| a.1.cmp(&b.1));

        let mut result = false;
        for (id, _) in id_indices {
            let mut unit = self.units[&id].borrow_mut();
            result |= unit.turn(&self);
        }
        result
    }

    fn check_occupation(&self, x: i32, y: i32) -> Occupation {
        let (index, tile) = self.map.get_tile_at_coords(x, y);
        if *tile == Tile::Floor {
            if let Some(unit_blocking) =
                self.units
                    .iter()
                    .find(|(_, unit_cell)| match unit_cell.try_borrow() {
                        Ok(unit) => unit.x == x && unit.y == y,
                        Err(_) => false,
                    })
            {
                return Occupation::Unit(index, *unit_blocking.0);
            } else {
                return Occupation::Free(index);
            }
        }

        Occupation::Wall(index)
    }

    fn unit_health_sum(&self) -> i32 {
        self.units
            .values()
            .map(|unit_cell| unit_cell.borrow().health)
            .sum()
    }
}

fn parse_input(input: Input) -> Cave {
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

    Cave::new(
        Map {
            tiles,
            dimension: 32,
        },
        units,
        unit_id - 1,
    )
}

#[aoc(15)]
fn solve_1(input: Input) {
    let cave = parse_input(input);
    cave.print();

    let mut turn_counter = 0;
    loop {
        turn_counter += 1;
        println!("==========\nTurn {}\n==========", turn_counter);

        // wtf rust, where is my do-while?
        if !cave.turn() {
            break;
        }

        cave.print();
    }

    let health_sum = cave.unit_health_sum();
    println!(
        "{} * {} = {}",
        turn_counter,
        health_sum,
        turn_counter * health_sum
    );
}

#[aoc(1)]
fn solve_2(input: Input) {}
