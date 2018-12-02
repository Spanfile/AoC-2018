use aoc_derive::aoc;
use counter::Counter;
use input;
use super::runner;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[aoc(2)]
fn solve_1(input: String) {
    let lines = input.lines();

    let mut twos = 0;
    let mut threes = 0;

    for s in lines {
        let mut letters = s.chars().collect::<Counter<_>>();
        let mut letter_counts: Vec<usize> = letters
            .most_common_ordered()
            .into_iter()
            .map(|(_, c)| c)
            .collect();

        if letter_counts.contains(&3) {
            threes += 1;
        }
        if letter_counts.contains(&2) {
            twos += 1;
        }
    }
    println!("{}", twos * threes);
}

#[aoc(2)]
fn solve_2(input: String) {
    let lines: Vec<&str> = input.lines().collect();

    for (i, s1) in lines[..lines.len()].iter().enumerate() {
        let check_lines = &lines[i + 1..];
        'outer: for s2 in check_lines {
            let mut differences = 0;
            let mut difference_at = 0;
            for i in 0..s1.len() {
                if (*s1).as_bytes()[i] != (*s2).as_bytes()[i] {
                    if differences == 1 {
                        continue 'outer;
                    }

                    differences += 1;
                    difference_at = i;
                }
            }

            let (start, mut end) = (*s1).split_at(difference_at);
            end = end.get(1..end.len()).unwrap();
            let id = start.to_owned() + end;
            println!("{}", id);
        }
    }
}
