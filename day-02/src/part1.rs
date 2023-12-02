use std::error::Error;

/**
 * Cubes:
 *
 * - red, green, blue
 * - some amount of each in the bag
 * - determine which games are possible given known amounts of each cube
 *
 * Steps:
 *
 * 1. iterate over game lines
 * 2. filter invalid games based on known dice limits
 * 3. extract and sum game ids from remaining lines
 *
 */

// pub enum Cube {
//     Red(u32),
//     Green(u32),
//     Blue(u32),
// }

#[derive(Debug)]
pub struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    pub fn new(red: u32, green: u32, blue: u32) -> Bag {
        Bag { red, green, blue }
    }

    pub fn from(game_details: &str) -> Bag {
        let rounds: Vec<&str> = game_details.split(';').map(|round| round.trim()).collect();

        let mut reds: u32 = 0;
        let mut greens: u32 = 0;
        let mut blues: u32 = 0;

        rounds.iter().for_each(|round| {
            let dies: Vec<_> = round.split(',').map(|die| die.trim()).collect();
            dies.iter().for_each(|die| {
                let die_details: Vec<_> = die.split(' ').collect();
                let count = die_details
                    .first()
                    .expect("should have a die count")
                    .parse::<u32>()
                    .expect("should be numeric die count");
                let die_label = die_details.last().expect("should have die label");

                match *die_label {
                    "red" => reds = count.max(reds),
                    "green" => greens = count.max(greens),
                    "blue" => blues = count.max(blues),
                    _ => (),
                }
            })
        });
        Bag::new(reds, greens, blues)
    }
}

pub fn run(input: &str) -> Result<u32, Box<dyn Error>> {
    let real_bag = Bag::new(12, 13, 14);

    let sum: u32 = input
        .lines()
        .filter_map(|line| {
            let game_parts: Vec<&str> = line.split(':').collect();

            let game_label = game_parts.first().expect("should have a game label");
            let game_id = game_label
                .split(' ')
                .last()
                .expect("should have game id")
                .parse::<u32>()
                .expect("should be numeric game id");

            let game_details = game_parts.last().expect("should have game details");

            let bag = Bag::from(game_details);

            if bag.red <= real_bag.red && bag.green <= real_bag.green && bag.blue <= real_bag.blue {
                return Some(game_id);
            }
            None
        })
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {

    use super::run;

    #[test]
    fn part1_example_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let expected = 8;

        assert_eq!(expected, run(input).expect("should return sum of game ids"));
    }
}
