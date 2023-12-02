use std::error::Error;

#[derive(Debug)]
pub struct GameRound {
    contents: Bag,
}

impl GameRound {
    pub fn from(round_str: &str) -> GameRound {
        let mut bag = Bag::empty();

        let dies: Vec<_> = round_str.split(',').map(|die| die.trim()).collect();

        dies.iter().for_each(|die| {
            let die_details: Vec<_> = die.split(' ').collect();
            let count = die_details
                .first()
                .expect("should have a die count")
                .parse::<u32>()
                .expect("should be numeric die count");
            let die_label = die_details.last().expect("should have die label");

            match *die_label {
                "red" => bag.red = count.max(bag.red),
                "green" => bag.green = count.max(bag.green),
                "blue" => bag.blue = count.max(bag.blue),
                _ => (),
            }
        });

        GameRound { contents: bag }
    }
}

#[derive(Debug)]
pub struct GameDetails {
    game_id: u32,
    bag: Bag,
}

impl GameDetails {
    pub fn from(game_record: &str) -> GameDetails {
        let game_parts: Vec<&str> = game_record.split(':').collect();

        let game_label = game_parts.first().expect("should have a game label");
        let game_id = game_label
            .split(' ')
            .last()
            .expect("should have game id")
            .parse::<u32>()
            .expect("should be numeric game id");

        let game_details = game_parts.last().expect("should have game details");

        GameDetails {
            game_id,
            bag: Bag::from(game_details),
        }
    }
}

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

    pub fn empty() -> Bag {
        Bag::new(0, 0, 0)
    }

    pub fn from(game_details: &str) -> Bag {
        let rounds: Vec<&str> = game_details.split(';').map(|round| round.trim()).collect();

        let mut bag = Bag::empty();

        rounds.iter().for_each(|round| {
            let game_round = GameRound::from(round);
            let round_bag = game_round.contents;

            bag.red = bag.red.max(round_bag.red);
            bag.green = bag.green.max(round_bag.green);
            bag.blue = bag.blue.max(round_bag.blue);
        });
        bag
    }
}

pub fn run(input: &str) -> Result<u32, Box<dyn Error>> {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let game_details = GameDetails::from(line);
            let bag = game_details.bag;

            bag.red * bag.green * bag.blue
        })
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn part2_example_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let expected = 2286;

        assert_eq!(
            expected,
            run(input).expect("should return sum of game powers")
        );
    }
}
