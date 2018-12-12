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
    pub pattern: Vec<bool>,
    pub next: bool,
}

impl Pots {
    pub fn new(initial: &str) -> Pots {
        let mut pots = HashMap::new();
        for (i, c) in initial.char_indices() {
            if c == '#' {
                pots.insert(i as i32, true);
            } else {
                pots.insert(i as i32, false);
            }
        }

        Pots { pots }
    }

    pub fn apply_rules(&mut self, rules: &Vec<Rule>) {
        let min = self.pots.keys().min().unwrap();
        let max = self.pots.keys().max().unwrap();
        let mut new_pots = self.pots.clone();

        for i in *min..=*max {
            for rule in rules {
                for check_i in -2..=2 {
                    if *self.pots.get(&(check_i + i)).unwrap_or(&false)
                        != rule.pattern[(check_i + 2) as usize]
                    {
                        new_pots.insert(i, *self.pots.get(&i).unwrap());
                        break;
                    }

                    new_pots.insert(i, rule.next);
                }
            }
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
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = s.split_whitespace().collect::<Vec<&str>>();
        let pattern = &args[0].chars().collect::<Vec<char>>();

        Ok(Rule {
            pattern: vec![
                pattern[0] == '#',
                pattern[1] == '#',
                pattern[2] == '#',
                pattern[3] == '#',
                pattern[4] == '#',
            ],
            next: args[1] == "#",
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

    let sum: i32 = pots
        .pots
        .iter()
        .filter_map(|(k, v)| if *v { Some(*k) } else { None })
        .sum();
    println!("{}", sum);
}

#[aoc(12)]
fn solve_2(input: Input) {}
