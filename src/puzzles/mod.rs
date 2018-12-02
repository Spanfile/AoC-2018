mod day1;
mod day2;
mod runner;

pub fn solve_latest() {
    solve_day(2);
}

pub fn solve_day(day: i32) {
    println!("Day {}", day);
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        _ => panic!("no such day: {}", day),
    };
}