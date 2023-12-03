use {{crate_name}}::part1::run;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../input/input1.txt");
    let result = run(input)?;

    println!("{:?}", result);
    Ok(())
}
