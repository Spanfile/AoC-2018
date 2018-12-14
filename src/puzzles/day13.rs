use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;
use std::collections::HashSet;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Cart {
    x: i32,
    y: i32,
    dir: Direction,
    turn_counter: i32,
}

enum Piece {
    Empty,
    Straight,
    Intersection,
    CurveL, // \
    CurveR, // /
}

struct Tracks {
    pieces: Vec<Piece>,
    width: i32,
}

impl Direction {
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Cart {
    pub fn new(x: i32, y: i32, dir: Direction) -> Cart {
        Cart {
            x,
            y,
            dir,
            turn_counter: 0,
        }
    }

    pub fn index(&self, width: i32) -> i32 {
        self.x + self.y * width
    }

    pub fn move_cart(&mut self, tracks: &Tracks) {
        match self.dir {
            Direction::Up => {
                self.y -= 1;
            }
            Direction::Down => {
                self.y += 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
        }

        self.update_dir(tracks);
    }

    fn update_dir(&mut self, tracks: &Tracks) {
        match tracks.get_piece_at(self.x, self.y) {
            Piece::CurveL => match self.dir {
                Direction::Up => self.dir = self.dir.turn_left(),
                Direction::Left => self.dir = self.dir.turn_right(),
                Direction::Down => self.dir = self.dir.turn_left(),
                Direction::Right => self.dir = self.dir.turn_right(),
            },
            Piece::CurveR => match self.dir {
                Direction::Up => self.dir = self.dir.turn_right(),
                Direction::Left => self.dir = self.dir.turn_left(),
                Direction::Down => self.dir = self.dir.turn_right(),
                Direction::Right => self.dir = self.dir.turn_left(),
            },
            Piece::Intersection => {
                self.turn_counter += 1;
                match self.turn_counter % 3 {
                    0 => {
                        self.dir = self.dir.turn_left();
                    }
                    2 => {
                        self.dir = self.dir.turn_right();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl Tracks {
    pub fn get_piece_at(&self, x: i32, y: i32) -> &Piece {
        &self.pieces[(x + self.width * y) as usize]
    }
}

#[aoc(13)]
fn solve_1(input: Input) {
    let mut pieces = Vec::new();
    let mut carts = Vec::new();
    let mut width = 0;

    let mut x = 0;
    let mut y = 0;
    for line in input.lines() {
        if width == 0 {
            width = line.len() as i32;
        }

        for c in line.chars() {
            pieces.push(match c {
                '-' => Piece::Straight,
                '|' => Piece::Straight,
                '/' => Piece::CurveR,
                '\\' => Piece::CurveL,
                '+' => Piece::Intersection,
                '<' => {
                    carts.push(Cart::new(x, y, Direction::Left));
                    Piece::Straight
                }
                '>' => {
                    carts.push(Cart::new(x, y, Direction::Right));
                    Piece::Straight
                }
                '^' => {
                    carts.push(Cart::new(x, y, Direction::Up));
                    Piece::Straight
                }
                'v' => {
                    carts.push(Cart::new(x, y, Direction::Down));
                    Piece::Straight
                }
                _ => Piece::Empty,
            });

            x += 1;
        }

        x = 0;
        y += 1;
    }

    let tracks = Tracks { pieces, width };

    'outer: loop {
        let mut new_carts = carts.clone();
        let mut new_cart_indices = HashSet::new();

        new_carts.sort_by(|a, b| a.index(width).cmp(&b.index(width)));
        for cart in &mut new_carts {
            cart.move_cart(&tracks);

            let index = cart.index(width);
            if !new_cart_indices.insert(index) {
                println!("CRASH!!!! at {},{}", cart.x, cart.y);
                break 'outer;
            }
        }

        carts = new_carts;
    }
}

#[aoc(13)]
fn solve_2(input: Input) {}
