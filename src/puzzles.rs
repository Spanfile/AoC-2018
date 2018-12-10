mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod runner;

pub fn solve_latest() {
    solve_day(10);
}

pub fn solve_day(day: i32) {
    println!("Day {}", day);
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        6 => day6::solve(),
        7 => day7::solve(),
        8 => day8::solve(),
        9 => day9::solve(),
        10 => day10::solve(),
        _ => panic!("no such day: {}", day),
    };
}
