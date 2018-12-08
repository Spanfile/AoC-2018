use super::runner;
use crate::input::{self, Input};
use aoc_derive::aoc;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

struct Node {
    pub children: Vec<Node>,
    pub metadata: Vec<i32>,
}

impl Node {
    pub fn value(&self) -> i32 {
        if self.children.len() == 0 {
            self.metadata.iter().sum()
        } else {
            let mut sum = 0;
            for i in &self.metadata {
                let index = i - 1;
                if index >= 0 && index < self.children.len() as i32 {
                    sum += self.children[index as usize].value();
                }
            }
            sum
        }
    }
}

fn build_tree<'a, I>(values: &mut I) -> Node
where
    I: Iterator<Item = i32>,
{
    let child_count = values.next().unwrap();
    let metadata_count = values.next().unwrap();
    let mut children = Vec::new();
    let mut metadata = Vec::new();

    for _ in 0..child_count {
        children.push(build_tree(values));
    }

    for _ in 0..metadata_count {
        metadata.push(values.next().unwrap());
    }

    Node { children, metadata }
}

fn sum_metadata(tree: &Node) -> i32 {
    let mut sum = tree.metadata.iter().sum();

    for child in &tree.children {
        sum += sum_metadata(&child);
    }

    sum
}

#[aoc(8)]
fn solve_1(input: Input) {
    let root = build_tree(&mut input.parse_split());
    println!("{}", sum_metadata(&root));
}

#[aoc(8)]
fn solve_2(input: Input) {
    let root = build_tree(&mut input.parse_split());
    println!("{}", root.value());
}
