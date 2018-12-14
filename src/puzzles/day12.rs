use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;
use std::collections::HashMap;
use std::str::FromStr;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[derive(Debug)]
struct Pots {
    pub pots: HashMap<i32, bool>,
}

#[derive(Debug)]
struct Rule {
    pub pattern: (bool, bool, bool, bool, bool),
    pub next: bool,
}

impl Pots {
    pub fn new(initial: &str) -> Pots {
        let mut pots = HashMap::new();
        for (i, c) in initial.char_indices() {
            pots.insert(i as i32, c == '#');
        }

        Pots { pots }
    }

    pub fn apply_rules(&mut self, rules: &Vec<Rule>) {
        let mut new_pots = HashMap::new();
        let min = *self.pots.keys().min().unwrap() - 1;
        let max = *self.pots.keys().max().unwrap() + 1;

        'outer: for i in min..=max {
            let mut check = Vec::new();

            for check_i in -2..=2 {
                check.push(*self.pots.get(&(i + check_i)).unwrap_or(&false));
            }

            for rule in rules {
                if !rule.next {
                    continue;
                }
                if (check[0], check[1], check[2], check[3], check[4]) == rule.pattern {
                    new_pots.insert(i, rule.next);
                    continue 'outer;
                }
            }

            new_pots.insert(i, false);
        }

        self.pots = new_pots;
    }

    pub fn print(&self) {
        let min = self.pots.keys().min().unwrap();
        let max = self.pots.keys().max().unwrap();

        for i in *min..=*max {
            if *self.pots.get(&i).unwrap() {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }

    pub fn score(&self) -> i32 {
        self.pots
            .iter()
            .filter_map(|(k, v)| if *v { Some(*k) } else { None })
            .sum()
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = s.split_whitespace().collect::<Vec<&str>>();
        let pattern = &args[0].chars().collect::<Vec<char>>();

        Ok(Rule {
            pattern: (
                pattern[0] == '#',
                pattern[1] == '#',
                pattern[2] == '#',
                pattern[3] == '#',
                pattern[4] == '#',
            ),
            next: args[2] == "#",
        })
    }
}

#[aoc(12)]
fn solve_1(_input: Input) {
    let mut pots = Pots::new("#........#.#.#...###..###..###.#..#....###.###.#.#...####..##..##.#####..##...#.#.....#...###.#.####");
    let rules = "#..## => .
##..# => #
..##. => .
.##.# => #
..... => .
..### => #
###.# => #
#.... => .
#.##. => #
.#.## => #
#...# => .
...## => .
###.. => #
.#..# => .
####. => .
....# => .
##### => #
.###. => .
#..#. => .
##... => #
.#... => #
#.#.# => .
..#.. => #
...#. => #
##.#. => .
.##.. => #
.#.#. => .
#.#.. => .
..#.# => #
#.### => .
##.## => .
.#### => #"
        .lines()
        .map(|l| l.parse::<Rule>().unwrap())
        .collect::<Vec<Rule>>();

    pots.print();

    for _ in 0..20 {
        pots.apply_rules(&rules);
        pots.print();
    }

    println!("{}", pots.score());
}

#[aoc(12)]
fn solve_2(_input: Input) {
    let mut pots = Pots::new("#........#.#.#...###..###..###.#..#....###.###.#.#...####..##..##.#####..##...#.#.....#...###.#.####");
    let rules = "#..## => .
##..# => #
..##. => .
.##.# => #
..... => .
..### => #
###.# => #
#.... => .
#.##. => #
.#.## => #
#...# => .
...## => .
###.. => #
.#..# => .
####. => .
....# => .
##### => #
.###. => .
#..#. => .
##... => #
.#... => #
#.#.# => .
..#.. => #
...#. => #
##.#. => .
.##.. => #
.#.#. => .
#.#.. => .
..#.# => #
#.### => .
##.## => .
.#### => #"
        .lines()
        .map(|l| l.parse::<Rule>().unwrap())
        .collect::<Vec<Rule>>();

    for _ in 0..500 {
        pots.apply_rules(&rules);
    }

    let score = pots.score() as i64 + 80 * (50000000000 as i64 - 500);
    println!("{}", score);
}
