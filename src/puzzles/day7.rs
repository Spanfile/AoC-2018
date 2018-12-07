use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;
use binary_heap_plus::*;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[derive(Debug)]
struct Step {
    pub id: char,
    pub prev: Vec<char>,
    pub next: Vec<char>,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = s.split_whitespace().collect::<Vec<_>>();
        let id = args[1].chars().collect::<Vec<char>>()[0];
        let next = args[7].chars().collect::<Vec<char>>()[0];

        Ok(Step {
            id,
            prev: Vec::new(),
            next: vec![next],
        })
    }
}

#[derive(Debug, Eq)]
struct Work {
    pub step_id: char,
    pub end: i32,
}

impl Ord for Work {
    fn cmp(&self, other: &Work) -> Ordering {
        match self.end.cmp(&other.end) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

impl PartialOrd for Work {
    fn partial_cmp(&self, other: &Work) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Work {
    fn eq(&self, other: &Work) -> bool {
        self.end == other.end
    }
}

fn build_steps(input: &str) -> HashMap<char, Step> {
    let lines = input.lines();
    let mut steps: HashMap<char, Step> = HashMap::new();
    for s in lines {
        let step = s.parse::<Step>().unwrap();
        let step_id = step.id;
        let next = step.next[0];

        match steps.entry(step.id) {
            Entry::Occupied(mut existing) => {
                existing.get_mut().next.extend(&step.next);
            }
            Entry::Vacant(entry) => {
                entry.insert(step);
            }
        };

        match steps.entry(next) {
            Entry::Occupied(mut e) => {
                e.get_mut().prev.push(step_id);
            }
            Entry::Vacant(entry) => {
                let key = *entry.key();
                entry.insert(Step {
                    id: key,
                    prev: vec![step_id],
                    next: Vec::new(),
                });
            }
        };
    }

    steps
}

fn step_duration(step: char, work_time: i32) -> i32 {
    i32::from(step as u8) - 64 + work_time
}

#[aoc(7)]
fn solve_1(input: Input) {
    let steps = build_steps(&input.get());
    let mut completed: HashSet<char> = HashSet::new();
    let mut final_order = Vec::new();

    let mut next_steps = binary_heap_plus::BinaryHeap::new_min();
    let first_steps = steps.values().filter(|s| s.prev.is_empty());

    for step in first_steps {
        next_steps.push(step.id);
    }

    while next_steps.peek().is_some() {
        let current = next_steps.pop().unwrap();
        completed.insert(current);
        final_order.push(current);

        for next_id in &steps[&current].next {
            if completed.is_superset(&steps[&next_id].prev.iter().cloned().collect::<HashSet<_>>())
            {
                next_steps.push(*next_id);
            }
        }
    }

    println!("{:?}", final_order.iter().collect::<String>());
}

#[aoc(7)]
fn solve_2(input: Input) {
    let workers = 4 + 1;
    let work_time = 60;

    let steps = build_steps(&input.get());
    let mut completed: HashSet<char> = HashSet::new();
    let mut active_work: BinaryHeap<Work> = BinaryHeap::new();
    let mut next_steps = BinaryHeap::new_min();

    for step in steps.values().filter(|s| s.prev.is_empty()) {
        next_steps.push(step.id);
    }

    let mut elapsed = 0;
    loop {
        while active_work.peek().is_some() && active_work.peek().unwrap().end == elapsed {
            let completed_work = active_work.pop().unwrap();
            completed.insert(completed_work.step_id);

            for next_id in &steps[&completed_work.step_id].next {
                if completed
                    .is_superset(&steps[&next_id].prev.iter().cloned().collect::<HashSet<_>>())
                {
                    next_steps.push(*next_id);
                }
            }
        }

        while active_work.len() < workers {
            match next_steps.pop() {
                Some(step_id) => {
                    active_work.push(Work {
                        step_id,
                        end: step_duration(step_id, work_time) + elapsed,
                    });
                }
                None => break,
            };
        }

        if completed == steps.keys().cloned().collect::<HashSet<char>>() {
            break;
        }

        elapsed += 1;
    }

    println!("{}", elapsed);
}
