fn main() {
    println!("Hello from main");

    let jobs = &[
        advent_of_code_2023::noop,
        // day_00_a::main,
        // day_00_b::main
    ];

    jobs.iter().for_each(|job| job());
}
