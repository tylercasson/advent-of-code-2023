use std::{collections::BTreeSet, error::Error, ops::Range};

const GALAXY_CHAR: char = '#';

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct Galaxy {
    id: usize,
    pos: Point,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct Point {
    x: i64,
    y: i64,
}

pub fn empty_cols(input: &str) -> Vec<usize> {
    let row = input.lines().next().expect("should have a first row");
    let width = row.len();

    let lines = input.lines();
    let mut empty_cols: Vec<usize> = vec![];

    (0..width).for_each(|i| {
        let col_length = lines
            .clone()
            .flat_map(|line| line.chars().enumerate().find(|&(j, _)| j == i))
            .map(|el| el.1)
            .fold(BTreeSet::new(), |mut acc, char| {
                acc.insert(char);
                acc
            })
            .len();

        if col_length == 1 {
            empty_cols.push(i);
        }
    });

    empty_cols
}

pub fn empty_rows(input: &str) -> Vec<usize> {
    let mut empty_rows: Vec<usize> = vec![];

    let lines = input.lines();
    lines.clone().enumerate().for_each(|(i, line)| {
        if !line.contains('#') {
            empty_rows.push(i);
        }
    });

    empty_rows
}

// TODO: use range intersections to speed up range testing
pub fn intersection(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
    let start = a.start.max(b.start);
    let end = a.end.min(b.end);

    if start < end {
        Some(start..end)
    } else {
        None
    }
}

pub fn min_path_length(
    a: Point,
    b: Point,
    empty_cols: &[usize],
    empty_rows: &[usize],
    expansion: i64,
) -> i64 {
    let max_x: i64 = a.x.max(b.x);
    let max_y: i64 = a.y.max(b.y);
    let min_x: i64 = a.x.min(b.x);
    let min_y: i64 = a.y.min(b.y);

    let x_set: BTreeSet<i64> = BTreeSet::from_iter(min_x..max_x);
    let y_set: BTreeSet<i64> = BTreeSet::from_iter(min_y..max_y);

    let mut empty_col_set: BTreeSet<i64> = BTreeSet::new();
    let mut empty_row_set: BTreeSet<i64> = BTreeSet::new();

    empty_cols.iter().for_each(|&col| {
        // increment by one since we're doing that for point coordinates
        empty_col_set.insert(col as i64 + 1);
    });

    empty_rows.iter().for_each(|&row| {
        // increment by one since we're doing that for point coordinates
        empty_row_set.insert(row as i64 + 1);
    });

    let x_expands = x_set.intersection(&empty_col_set).count() as i64;
    let y_expands = y_set.intersection(&empty_row_set).count() as i64;

    let x_norms = x_set.difference(&empty_col_set).count() as i64;
    let y_norms = y_set.difference(&empty_row_set).count() as i64;

    // apply expansion term to empty regions
    let x_extra = x_expands * expansion;
    let y_extra = y_expands * expansion;

    x_norms + y_norms + x_extra + y_extra
}

pub fn custom_run(input: &str, expansion: i64) -> Result<String, Box<dyn Error>> {
    let empty_cols = empty_cols(input);
    let empty_rows = empty_rows(input);

    let space = input;
    let lines: Vec<&str> = space.lines().collect();

    let width = lines.first().unwrap().len();
    let height = lines.len();

    let mut id: usize = 0;

    let galaxies: Vec<Galaxy> = space
        .chars()
        .enumerate()
        .flat_map(|(i, char)| {
            let x = i % (width + 1) + 1;
            let y = (i / (width + 1)) % height + 1;
            if char == GALAXY_CHAR {
                id += 1;
                return Some(Galaxy {
                    id,
                    pos: Point {
                        x: x as i64,
                        y: y as i64,
                    },
                });
            }
            None
        })
        .collect();

    let mut paths: BTreeSet<(&Galaxy, &Galaxy)> = BTreeSet::new();

    'outer: for left in galaxies.iter() {
        'inner: for right in galaxies.iter() {
            if left == right {
                continue 'outer;
            }
            let mut items = [left, right];
            // sort to avoid repeats
            items.sort_by(|a, b| a.id.cmp(&b.id));
            let (left, right) = (items.first().unwrap(), items.last().unwrap());

            if paths.contains(&(left, right)) {
                break 'outer;
            }

            paths.insert((left, right));
        }
    }

    let path_sum: i64 = paths
        .iter()
        .map(|(a, b)| min_path_length(a.pos, b.pos, &empty_cols, &empty_rows, expansion))
        .sum();

    Ok(path_sum.to_string())
}

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    custom_run(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_2() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let expected = "374".to_string();

        assert_eq!(
            expected,
            custom_run(input, 2).expect("should return expected value")
        );
    }

    #[test]
    fn example_input_10() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let expected = "1030".to_string();

        assert_eq!(
            expected,
            custom_run(input, 10).expect("should return expected value")
        );
    }

    #[test]
    fn example_input_100() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let expected = "8410".to_string();

        assert_eq!(
            expected,
            custom_run(input, 100).expect("should return expected value")
        );
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/input1.txt");

        // verify new strat works for part 1 answer
        let expected = "9545480".to_string();

        assert_eq!(
            expected,
            custom_run(input, 2).expect("should return expected value")
        );
    }
}
