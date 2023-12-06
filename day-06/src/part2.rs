use std::error::Error;

#[derive(Debug)]
pub struct Race {
    time: f64,
    dist: f64,
}

impl Race {
    pub fn winning_options(&self) -> f64 {
        // dist = time * x - x^2
        let a = -1.0;
        let b = self.time;
        let c = -(self.dist + 1.0);
        let disc = b * b - 4.0 * a * c;

        let x1 = (-b + f64::sqrt(disc)) / (2.0 * a);
        let x2 = (-b - f64::sqrt(disc)) / (2.0 * a);

        // inclusive
        x2.floor() - x1.ceil() + 1.0
    }
}

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let mut lines = input.lines();
    let time: f64 = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse::<f64>()
        .unwrap();

    let dist: f64 = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse::<f64>()
        .unwrap();

    let race = Race { time, dist };

    let options: f64 = race.winning_options();

    Ok((options as u64).to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let expected = "71503".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
