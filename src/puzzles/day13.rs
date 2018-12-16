use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, Copy)]
struct Cart {
    id: i32,
    x: i32,
    y: i32,
    dir: Direction,
    tracks_width: i32,
    turn_counter: i32,
}

#[derive(Debug)]
enum Piece {
    Empty,
    StraightLR,
    StraightUD,
    Intersection,
    CurveL, // \
    CurveR, // /
}

struct Tracks {
    pieces: Vec<Piece>,
    width: i32,
}

impl Direction {
    pub fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Cart {
    pub fn new(id: i32, x: i32, y: i32, dir: Direction, tracks_width: i32) -> Cart {
        Cart {
            id,
            x,
            y,
            dir,
            tracks_width,
            turn_counter: 0,
        }
    }

    pub fn index(&self) -> i32 {
        self.x + self.y * self.tracks_width
    }

    pub fn move_cart(&mut self, tracks: &Tracks) {
        // println!("moving cart {:?}", self);
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
                match self.turn_counter % 3 {
                    0 => {
                        self.dir = self.dir.turn_left();
                    }
                    2 => {
                        self.dir = self.dir.turn_right();
                    }
                    _ => {}
                }
                self.turn_counter += 1;
            }
            Piece::StraightLR => match self.dir {
                Direction::Up => println!("wtf I'm on a left-right track going up"),
                Direction::Down => println!("wtf I'm on a left-right track going down"),
                _ => {}
            },
            Piece::StraightUD => match self.dir {
                Direction::Left => println!("wtf I'm on a up-down track going left"),
                Direction::Right => println!("wtf I'm on a up-down track going right"),
                _ => {}
            },
            _ => {}
        }
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.id == other.id
    }
}

impl Hash for Cart {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Tracks {
    pub fn get_piece_at(&self, x: i32, y: i32) -> &Piece {
        // println!("getting piece at {},{}", x, y);
        &self.pieces[(x + self.width * y) as usize]
    }

    // pub fn print(&self, crash_x: i32, crash_y: i32) {
    //     for y in 0..self.width {
    //         print!("{:03} ", y);
    //         for x in 0..self.width {
    //             if x == crash_x && y == crash_y {
    //                 print!("X");
    //                 continue;
    //             }

    //             print!(
    //                 "{}",
    //                 match self.get_piece_at(x, y) {
    //                     Piece::StraightLR => "-",
    //                     Piece::StraightUD => "|",
    //                     Piece::Intersection => "+",
    //                     Piece::CurveL => "\\",
    //                     Piece::CurveR => "/",
    //                     Piece::Empty => " ",
    //                 }
    //             );
    //         }
    //         println!();
    //     }
    // }
}

fn build_tracks_carts(input: Input) -> (Tracks, HashMap<i32, Cart>) {
    let mut pieces = Vec::new();
    let mut carts = HashMap::new();
    let width: i32 = 150;

    let mut x = 0;
    let mut y = 0;
    let mut cart_id = 0;

    for c in input.get().chars() {
        if c == '\n' {
            y += 1;
            x = 0;
            continue;
        }

        pieces.push(match c {
            '-' => Piece::StraightLR,
            '|' => Piece::StraightUD,
            '/' => Piece::CurveR,
            '\\' => Piece::CurveL,
            '+' => Piece::Intersection,
            '<' => {
                carts.insert(cart_id, Cart::new(cart_id, x, y, Direction::Left, width));
                cart_id += 1;
                Piece::StraightLR
            }
            '>' => {
                carts.insert(cart_id, Cart::new(cart_id, x, y, Direction::Right, width));
                cart_id += 1;
                Piece::StraightLR
            }
            '^' => {
                carts.insert(cart_id, Cart::new(cart_id, x, y, Direction::Up, width));
                cart_id += 1;
                Piece::StraightUD
            }
            'v' => {
                carts.insert(cart_id, Cart::new(cart_id, x, y, Direction::Down, width));
                cart_id += 1;
                Piece::StraightUD
            }
            _ => Piece::Empty,
        });

        x += 1;
    }

    let tracks = Tracks { pieces, width };
    (tracks, carts)
}

#[aoc(13)]
fn solve_1(input: Input) {
    let (tracks, mut carts) = build_tracks_carts(input);
    let mut _tick = 0;

    'outer: loop {
        let carts_clone = carts.clone();
        let mut cart_ids = carts_clone.values().collect::<Vec<&Cart>>();
        cart_ids.sort_by(|a, b| a.index().cmp(&b.index()));
        for id in cart_ids.iter().map(|c| c.id) {
            let mut cart = carts[&id];
            cart.move_cart(&tracks);
            carts.insert(id, cart);

            let index = cart.index();
            if carts
                .values()
                .any(|c| c.id != cart.id && c.index() == index)
            {
                // tracks.print(cart.x, cart.y);
                println!("CRASH!!!! at {},{}", cart.x, cart.y);
                // println!("{:?}", cart);
                // println!("{:?}", tracks.get_piece_at(cart.x, cart.y));
                // println!("tick {}", _tick);
                break 'outer;
            }
        }

        _tick += 1;
    }
}

#[aoc(13)]
fn solve_2(input: Input) {
    let (tracks, mut carts) = build_tracks_carts(input);
    let mut crashed_carts = HashSet::new();
    let mut _tick = 0;

    loop {
        let carts_clone = carts.clone();
        let mut cart_ids = carts_clone.values().collect::<Vec<&Cart>>();
        cart_ids.sort_by(|a, b| a.index().cmp(&b.index()));

        let mut carts_moved = 0;
        let mut last_cart_moved = None;

        // println!("tick {}", tick);
        for id in cart_ids.iter().map(|c| c.id) {
            let mut cart = carts[&id];
            if crashed_carts.contains(&cart.id) {
                continue;
            }

            cart.move_cart(&tracks);
            carts.insert(id, cart);

            carts_moved += 1;
            last_cart_moved = Some(cart);
            // println!("moved {}", cart.id);

            let index = cart.index();
            let crashed_with = carts
                .values()
                .find(|c| c.id != cart.id && c.index() == index && !crashed_carts.contains(&c.id));
            if crashed_with.is_some() {
                let crashed_with = crashed_with.unwrap();
                crashed_carts.insert(cart.id);
                crashed_carts.insert(crashed_with.id);
                println!(
                    "CRASH!!!! between {} and {} at {},{} on tick {}",
                    cart.id, crashed_with.id, cart.x, cart.y, _tick
                );
            }
        }

        if carts_moved == 1 {
            let last_cart_moved = last_cart_moved.unwrap();
            println!("only cart moved on tick {}: {:?}", _tick, last_cart_moved);
            println!(
                "{:?}",
                tracks.get_piece_at(last_cart_moved.x, last_cart_moved.y)
            );
            break;
        }

        _tick += 1;
    }
}
