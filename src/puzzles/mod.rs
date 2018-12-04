mod day1;
mod day2;
mod day3;
mod day4;
mod runner;

pub fn solve_latest() {
    solve_day(4);
}

pub fn solve_day(day: i32) {
    println!("Day {}", day);
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        _ => panic!("no such day: {}", day),
    };
}
