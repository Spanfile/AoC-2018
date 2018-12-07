use std::time::Instant;

pub fn run_solutions(solve_1: &Fn() -> (), solve_2: &Fn() -> ()) {
    println!("1:");
    timeit(solve_1);

    println!("2:");
    timeit(solve_2);
}

fn timeit(func: &Fn() -> ()) {
    let start = Instant::now();
    func();
    let duration = start.elapsed();
    println!(
        "Time taken: {}ms",
        duration.as_micros() as f64 / f64::from(1000)
    );
}
