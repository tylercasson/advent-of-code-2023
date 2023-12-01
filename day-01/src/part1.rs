use std::error::Error;

pub fn extract_number_literal(input: &str) -> Result<char, Box<dyn Error>> {
    if let Some(value) = input.chars().find(|char| char.is_numeric()) {
        return Ok(value);
    }
    let message = format!("Error extracting number literal from input: {}", input);
    Err(Box::from(message))
}

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

#[cfg(test)]
mod tests {

    use super::run;

    #[test]
    fn part1_example_input() {
        let expected = 142.to_string();
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(expected, run(input).unwrap());
    }
}
