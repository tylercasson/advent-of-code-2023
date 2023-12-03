use std::error::Error;

const VOID_CHAR: char = '.';

#[derive(Debug, PartialEq, Clone)]
pub struct BoundingBox {
    origin: EnginePosition,
    width: i32,
    height: i32,
}

impl BoundingBox {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> BoundingBox {
        BoundingBox {
            origin: EnginePosition { x, y },
            width,
            height,
        }
    }

    pub fn intersects(self: &BoundingBox, other: &BoundingBox) -> bool {
        let min_x_1 = self.origin.x;
        let min_x_2 = other.origin.x;

        let max_x_1 = self.origin.x + self.width;
        let max_x_2 = other.origin.x + other.width;

        let min_y_1 = self.origin.y;
        let min_y_2 = other.origin.y;

        let max_y_1 = self.origin.y + self.height;
        let max_y_2 = other.origin.y + other.height;

        min_x_1 < max_x_2 && min_x_2 < max_x_1 && min_y_1 < max_y_2 && min_y_2 < max_y_1
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnginePosition {
    x: i32,
    y: i32,
}

impl EnginePosition {
    pub fn new(index: i32, line_num: i32) -> EnginePosition {
        EnginePosition {
            x: index,
            y: line_num,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EngineNumber {
    position: EnginePosition,
    value: i32,
    length: i32,
}

impl EngineNumber {
    pub fn new(value: i32, length: i32, index: i32, line_num: i32) -> EngineNumber {
        let position = EnginePosition::new(index, line_num);
        EngineNumber {
            length,
            position,
            value,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EngineSymbol {
    position: EnginePosition,
}

impl EngineSymbol {
    pub fn new(index: i32, line_num: i32) -> EngineSymbol {
        let position = EnginePosition::new(index, line_num);
        EngineSymbol { position }
    }
}

#[derive(Debug, PartialEq)]
pub struct EngineData {
    numbers: Vec<EngineNumber>,
    symbols: Vec<EngineSymbol>,
}

impl EngineData {
    pub fn new(numbers: Vec<EngineNumber>, symbols: Vec<EngineSymbol>) -> EngineData {
        EngineData { numbers, symbols }
    }

    pub fn from(input: &str) -> EngineData {
        let mut numbers: Vec<EngineNumber> = Vec::new();
        let mut symbols: Vec<EngineSymbol> = Vec::new();

        input.lines().enumerate().for_each(|(line_num, line)| {
            let mut num_chars: Vec<char> = Vec::new();

            line.chars().enumerate().for_each(|(i, char)| {
                if char.is_ascii_digit() {
                    num_chars.push(char);
                } else {
                    if char != VOID_CHAR {
                        symbols.push(EngineSymbol::new(i as i32, line_num as i32));
                    }
                    if let Ok(number) = num_chars.iter().collect::<String>().parse::<i32>() {
                        numbers.push(EngineNumber::new(
                            number,
                            num_chars.len() as i32,
                            (i - num_chars.len()) as i32,
                            line_num as i32,
                        ));
                    }
                    num_chars.clear();
                }
            });

            if let Ok(number) = num_chars.iter().collect::<String>().parse::<i32>() {
                numbers.push(EngineNumber::new(
                    number,
                    num_chars.len() as i32,
                    (line.len() - num_chars.len()) as i32,
                    line_num as i32,
                ));
            }
        });

        EngineData::new(numbers, symbols)
    }
}

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let data = EngineData::from(input);
    let mut sum: i32 = 0;

    data.numbers.iter().for_each(|number| {
        let pos = &number.position;
        let origin = EnginePosition::new(pos.x, pos.y);
        let nbox = BoundingBox {
            origin,
            height: 1,
            width: number.length,
        };

        let num_symbols: Vec<_> = data
            .symbols
            .iter()
            .filter(|symbol| {
                let pos = &symbol.position;
                let origin = EnginePosition::new(pos.x - 1, pos.y - 1);
                let sbox = BoundingBox {
                    origin,
                    height: 3,
                    width: 3,
                };

                sbox.intersects(&nbox)
            })
            .collect();

        if !num_symbols.is_empty() {
            sum += number.value;
        }
    });

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use crate::part1::EngineData;

    use super::{run, BoundingBox};

    #[test]
    fn part1_example_input() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let expected = "4361".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }

    #[test]
    fn extract_engine_details() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let expected: Vec<i32> = vec![467, 114, 35, 633, 617, 58, 592, 755, 664, 598];
        let expected_symbol_pos: Vec<(i32, i32)> =
            vec![(3, 1), (6, 3), (3, 4), (5, 5), (3, 8), (5, 8)];
        let data = EngineData::from(input);
        let numbers: Vec<i32> = data.numbers.iter().map(|number| number.value).collect();
        let symbol_positions: Vec<(i32, i32)> = data
            .symbols
            .iter()
            .map(|symbol| (symbol.position.x, symbol.position.y))
            .collect();

        assert_eq!(expected, numbers);
        assert_eq!(expected_symbol_pos, symbol_positions);
    }

    #[test]
    fn boxes_intersect() {
        let box_1 = BoundingBox::new(0, 0, 3, 1);
        let box_2 = BoundingBox::new(0, 0, 3, 3);

        assert!(box_1.intersects(&box_2));
        assert!(box_2.intersects(&box_1));
    }

    #[test]
    fn boxes_do_not_intersect() {
        let box_1 = BoundingBox::new(-1, -1, 3, 3);
        let box_2 = BoundingBox::new(2, 2, 3, 3);

        assert!(!box_1.intersects(&box_2));
        assert!(!box_2.intersects(&box_1));
    }
}
