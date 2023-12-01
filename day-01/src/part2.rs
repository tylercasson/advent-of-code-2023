use std::{
    cmp::{self, Ordering},
    collections::HashMap,
    error::Error,
};

/*
   Basic Algorithm

   For each line:

        1. Find first number-literal index
        2. Find first number-word index
        3. Find last number-literal index
        4. Find last number-word index
        5. Determine earliest and latest for each side
        6. Combine into number pair
*/

pub enum Direction {
    LTR,
    RTL,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct NumberResult {
    pub index: usize,
    pub value: String,
}

impl NumberResult {
    pub fn new(index: usize, value: u32) -> NumberResult {
        NumberResult {
            index,
            value: value.to_string(),
        }
    }

    pub fn blank() -> NumberResult {
        NumberResult {
            index: usize::MAX,
            value: String::from(""),
        }
    }
}

pub fn reverse(string: String) -> String {
    string.chars().rev().collect()
}

/// Extracts the first literal number from an `input` string slice in `direction`
///
/// # Examples
///
/// ```
/// # use day_01::part2::{extract_number_literal, Direction, NumberResult};
/// let input = "asztwo32nine8rrew";
/// let left = Some(NumberResult::new(6, 3));
/// let right = Some(NumberResult::new(12, 8));
/// assert_eq!(left, extract_number_literal(input, Direction::LTR));
/// assert_eq!(right, extract_number_literal(input, Direction::RTL));
/// ```
pub fn extract_number_literal(input: &str, direction: Direction) -> Option<NumberResult> {
    let mut test_input: String = input.to_string();
    let mut index_correction = 0;
    if let Direction::RTL = direction {
        test_input = reverse(test_input);
        index_correction = input.len() - 1;
    }

    for (i, char) in test_input.char_indices() {
        if char.is_numeric() {
            let mut index = i;
            if let Direction::RTL = direction {
                index = index_correction - index;
            }
            return Some(NumberResult::new(
                index,
                char.to_string().parse::<u32>().unwrap(),
            ));
        }
    }
    None
}

/// Extracts the first number from an `input` string in `direction` based on
/// a word map (e.g. `("one", 1), ("two", 2), ("three", 3)...`).
///
/// # Examples
///
/// ```
/// # use day_01::part2::{extract_number_word, Direction, NumberResult};
/// let input = "asztwo32nine8rrew";
/// let left = Some(NumberResult::new(3, 2));
/// let right = Some(NumberResult::new(8, 9));
/// assert_eq!(left, extract_number_word(input, Direction::LTR));
/// assert_eq!(right, extract_number_word(input, Direction::RTL));
/// ```
pub fn extract_number_word(input: &str, direction: Direction) -> Option<NumberResult> {
    let number_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut offset: usize = 0;
    let mut test_input: String = input.to_string();
    let mut index_correction = 0;

    if let Direction::RTL = direction {
        test_input = reverse(test_input);
        index_correction = input.len() - 1;
    }

    while offset < test_input.len() {
        for (&word, &value) in number_map.iter() {
            let end = cmp::min(offset + word.len(), test_input.len());
            let curr = &test_input[offset..end];
            let mut test_word: String = word.to_string();
            if let Direction::RTL = direction {
                test_word = reverse(word.to_string());
            }
            if curr == test_word {
                let mut index = offset;
                if let Direction::RTL = direction {
                    index = index_correction - index - word.len() + 1;
                }
                return Some(NumberResult::new(index, value));
            }
        }
        offset += 1;
    }

    None
}

/// Extracts a two-digit number from an `input` string
/// slice comprised of the first and last extracted number.
///
/// # Examples
///
/// ```
/// # use day_01::part2::{extract_number_pair};
/// let input = "asztwo32nine8rrew";
/// let pair = "28";
/// assert_eq!(pair, extract_number_pair(input));
/// ```
pub fn extract_number_pair(input: &str) -> String {
    let left_num = extract_number_literal(input, Direction::LTR);
    let right_num = extract_number_literal(input, Direction::RTL);

    let left_word = extract_number_word(input, Direction::LTR);
    let right_word = extract_number_word(input, Direction::RTL);

    let left_number = match (&left_num, &left_word) {
        (Some(num), Some(word)) => match num.index.cmp(&word.index) {
            Ordering::Greater => left_word,
            Ordering::Less => left_num,
            Ordering::Equal => Some(NumberResult::blank()),
        },
        (Some(_), None) => left_num,
        (None, Some(_)) => left_word,
        _ => None,
    };

    let right_number = match (&right_num, &right_word) {
        (Some(num), Some(word)) => match num.index.cmp(&word.index) {
            Ordering::Greater => right_num,
            Ordering::Less => right_word,
            Ordering::Equal => Some(NumberResult::blank()),
        },
        (Some(_), None) => right_num,
        (None, Some(_)) => right_word,
        _ => None,
    };

    let left = left_number.unwrap_or(NumberResult::blank());
    let right = right_number.unwrap_or(NumberResult::blank());
    let combo = format!("{}{}", left.value, right.value);

    combo
}

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let mut sum = 0;

    for line in input.lines() {
        let pair = extract_number_pair(line);

        if let Ok(value) = pair.parse::<u32>() {
            sum += value;
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {

    use super::{
        extract_number_literal, extract_number_pair, extract_number_word, run, Direction,
        NumberResult,
    };

    #[test]
    fn part2_example_input() {
        let expected = 281.to_string();
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(expected, run(input).unwrap());
    }

    #[test]
    fn extracts_number_literal_ltr() {
        let input = "zphgdcz2nqsm";
        let expected: Option<NumberResult> = Some(NumberResult::new(7, 2));

        assert_eq!(expected, extract_number_literal(input, Direction::LTR));
    }

    #[test]
    fn extracts_number_literal_rtl() {
        let input = "zphgdcz2nqsm";
        let expected: Option<NumberResult> = Some(NumberResult::new(7, 2));

        assert_eq!(expected, extract_number_literal(input, Direction::RTL));
    }

    #[test]
    fn extracts_number_words_ltr() {
        let input = "zphgdcznqsm2";
        let expected: Option<NumberResult> = None;

        assert_eq!(expected, extract_number_word(input, Direction::LTR))
    }

    #[test]
    fn extracts_number_words_rtl() {
        let input = "zphgdczsixqrm";
        let expected: Option<NumberResult> = Some(NumberResult::new(7, 6));

        assert_eq!(expected, extract_number_word(input, Direction::RTL))
    }

    #[test]
    fn extracts_number_pair_mixed() {
        let input = "two1nine";
        let expected = "29".to_string();

        assert_eq!(expected, extract_number_pair(input))
    }

    #[test]
    fn extracts_number_pair_words_only() {
        let input = "eightwothree";
        let expected = "83".to_string();

        assert_eq!(expected, extract_number_pair(input))
    }

    #[test]
    fn extracts_number_pair_one_number_only() {
        let input = "zphgdcznqsm2";
        let expected = "22".to_string();

        assert_eq!(expected, extract_number_pair(input))
    }
}
