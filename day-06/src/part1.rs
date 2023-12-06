use std::{error::Error, iter::zip};

#[derive(Debug)]
pub struct Race {
    time: f32,
    dist: f32,
}

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let mut lines = input.lines();
    let times: Vec<f32> = lines
        .next()
        .unwrap()
        .split(' ')
        .filter(|el| !el.is_empty())
        .flat_map(|time| time.parse::<f32>())
        .collect();

    let dists: Vec<f32> = lines
        .next()
        .unwrap()
        .split(' ')
        .filter(|el| !el.is_empty())
        .flat_map(|dist| dist.parse::<f32>())
        .collect();

    let races: Vec<Race> = zip(times, dists)
        .map(|(time, dist)| Race { time, dist })
        .collect();

    let options: f32 = races
        .iter()
        .map(|race| {
            // dist = time * x - x^2
            let a = -1.0;
            let b = race.time;
            let c = -(race.dist + 1.0);
            let disc = b * b - 4.0 * a * c;

            let x1 = (-b + f32::sqrt(disc)) / (2.0 * a);
            let x2 = (-b - f32::sqrt(disc)) / (2.0 * a);

            // inclusive
            x2.floor() - x1.ceil() + 1.0
        })
        .product();

    Ok((options as u32).to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let expected = "288".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
