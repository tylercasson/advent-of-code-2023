use std::error::Error;

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    todo!("Implement part 2");
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example_input() {
        let input = "";

        let expected = "".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
