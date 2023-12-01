use std::error::Error;

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        if !line.is_empty() {
            let first = line.chars().find(|char| char.is_numeric()).unwrap();
            let last = line.chars().rev().find(|char| char.is_numeric()).unwrap();

            let num_string = format!("{}{}", first, last);

            if let Ok(number) = num_string.parse::<u32>() {
                sum += number;
            }
        }
    }

    Ok(sum.to_string())
}
