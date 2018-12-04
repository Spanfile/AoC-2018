use super::runner;
use aoc_derive::aoc;
use input;
use sorted_list::SortedList;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve() {
    runner::run_solutions(&do_solve_1, &do_solve_2);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Timestamp {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
}

impl FromStr for Timestamp {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x: &[_] = &['[', ']'];
        let args = s
            .trim_matches(x)
            .split_whitespace()
            .into_iter()
            .collect::<Vec<&str>>();

        let date_args = args[0].split('-').collect::<Vec<&str>>();
        let time_args = args[1].split(':').collect::<Vec<&str>>();

        let year = date_args[0].parse::<i32>().unwrap();
        let month = date_args[1].parse::<i32>().unwrap();
        let day = date_args[2].parse::<i32>().unwrap();
        let hour = time_args[0].parse::<i32>().unwrap();
        let minute = time_args[1].parse::<i32>().unwrap();

        Ok(Timestamp {
            year,
            month,
            day,
            hour,
            minute,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Event {
    Begins,
    FallsAsleep,
    WakesUp,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Record {
    pub timestamp: Timestamp,
    pub guard_id: i32,
    pub event: Event,
}

impl FromStr for Record {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = s.split("] ").collect::<Vec<&str>>();

        let timestamp = args[0].parse::<Timestamp>().unwrap();
        let mut guard_id = -1;
        let mut event = Event::Begins;

        let event_args = args[1].split_whitespace().collect::<Vec<&str>>();
        match event_args[0] {
            "Guard" => guard_id = event_args[1].trim_matches('#').parse::<i32>().unwrap(),
            "falls" => event = Event::FallsAsleep,
            "wakes" => event = Event::WakesUp,
            &_ => panic!("unknown record: {}", s),
        };

        Ok(Record {
            timestamp,
            guard_id,
            event,
        })
    }
}

#[aoc(4)]
fn solve_1(input: String) {
    let lines = input.lines();

    let mut records: SortedList<Timestamp, Record> = SortedList::new();
    for s in lines {
        let record = s.parse::<Record>().unwrap();
        records.insert(record.timestamp, record);
    }

    let mut guard_sleep_times: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    let mut current_guard = -1;
    let mut fell_asleep_at = -1;

    for (timestamp, record) in records.iter() {
        match record.event {
            Event::Begins => current_guard = record.guard_id,
            Event::FallsAsleep => {
                // println!(
                //     "guard {} falls asleep at {}:{}",
                //     current_guard, timestamp.hour, timestamp.minute
                // );
                fell_asleep_at = timestamp.minute;
            }
            Event::WakesUp => {
                // println!(
                //     "{} fell asleep at {} and woke up at {}",
                //     current_guard, fell_asleep_at, timestamp.minute
                // );

                let mut sleep_minutes: &mut HashMap<i32, i32> = guard_sleep_times
                    .entry(current_guard)
                    .or_insert(HashMap::new());
                for minute in fell_asleep_at..timestamp.minute {
                    let mut sleep_minute = sleep_minutes.entry(minute).or_insert(0);
                    *sleep_minute += 1;
                }
            }
        }
    }

    let mut most_sleeping_guard = (-1, -1);

    for (guard_id, sleep_minutes) in &guard_sleep_times {
        let total_sleep_time = sleep_minutes.values().sum();
        if total_sleep_time as i32 >= most_sleeping_guard.1 {
            most_sleeping_guard = (*guard_id, total_sleep_time);
        }
    }

    let mut most_slept_minute = (-1, -1);
    for (minute, count) in guard_sleep_times.get(&most_sleeping_guard.0).unwrap() {
        if count > &most_slept_minute.1 {
            most_slept_minute = (*minute, *count);
        }
    }

    println!(
        "{} ({}) * {} ({}) = {}",
        most_sleeping_guard.0,
        most_sleeping_guard.1,
        most_slept_minute.0,
        most_slept_minute.1,
        most_sleeping_guard.0 * most_slept_minute.0
    );
}

#[aoc(4)]
fn solve_2(input: String) {
    let lines = input.lines();

    let mut records: SortedList<Timestamp, Record> = SortedList::new();
    for s in lines {
        let record = s.parse::<Record>().unwrap();
        records.insert(record.timestamp, record);
    }

    let mut guard_sleep_times: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    let mut current_guard = -1;
    let mut fell_asleep_at = -1;
    let mut most_slept = -1;
    let mut most_slept_guard_minute = (-1, -1);

    for (timestamp, record) in records.iter() {
        match record.event {
            Event::Begins => current_guard = record.guard_id,
            Event::FallsAsleep => {
                // println!(
                //     "guard {} falls asleep at {}:{}",
                //     current_guard, timestamp.hour, timestamp.minute
                // );
                fell_asleep_at = timestamp.minute;
            }
            Event::WakesUp => {
                // println!(
                //     "{} fell asleep at {} and woke up at {}",
                //     current_guard, fell_asleep_at, timestamp.minute
                // );

                for minute in fell_asleep_at..timestamp.minute {
                    let mut guard_sleep_time =
                        guard_sleep_times.entry(minute).or_insert(HashMap::new());
                    let mut slept_this_minute = guard_sleep_time.entry(current_guard).or_insert(0);
                    *slept_this_minute += 1;

                    if *slept_this_minute > most_slept {
                        most_slept = *slept_this_minute;
                        most_slept_guard_minute = (current_guard, minute);
                    }
                }
            }
        }
    }

    println!(
        "{} * {} = {}",
        most_slept_guard_minute.0,
        most_slept_guard_minute.1,
        most_slept_guard_minute.0 * most_slept_guard_minute.1
    );
}
