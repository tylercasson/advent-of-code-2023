use std::error::Error;

use day_01::part1;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../input/input1.txt");
    let result = part1::run(input)?;

    println!("{:?}", result);
    Ok(())
}
