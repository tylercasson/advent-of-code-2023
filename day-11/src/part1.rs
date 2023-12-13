use std::{
    collections::{BTreeSet, HashSet},
    error::Error,
};

const GALAXY_CHAR: char = '#';

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct Galaxy {
    id: usize,
    pos: Point,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct Point {
    x: i32,
    y: i32,
}

pub fn expand_space(input: &str) -> Result<String, Box<dyn Error>> {
    let row = input.lines().next().expect("should have a first row");
    let width = row.len();

    let lines = input.lines();
    let mut new_lines = vec![];

    lines.clone().for_each(|line| {
        new_lines.push(line);
        if !line.contains('#') {
            new_lines.push(line);
        }
    });

    let mut empty_cols: Vec<usize> = vec![];

    (0..width).for_each(|i| {
        let col_length = lines
            .clone()
            .flat_map(|line| line.chars().enumerate().find(|&(j, _)| j == i))
            .map(|el| el.1)
            .fold(HashSet::new(), |mut acc, char| {
                acc.insert(char);
                acc
            })
            .len();

        if col_length == 1 {
            empty_cols.push(i);
        }
    });

    let new_lines = new_lines.join("\n");
    let mut new_text = String::new();

    new_lines.chars().enumerate().for_each(|(i, char)| {
        let col = i % (width + 1);

        new_text.push(char);
        if empty_cols.contains(&col) {
            new_text.push(char);
        }
    });

    Ok(new_text)
}

pub fn min_path_length(a: Point, b: Point) -> u32 {
    let mut count = 0;
    let mut last = a;
    let mut next = Point { x: a.x, y: a.y };
    let x2 = b.x;
    let y2 = b.y;

    'outer: loop {
        if count > 0 && last == b {
            break 'outer;
        }

        let x1 = next.x;
        let y1 = next.y;

        // update last point
        last = next;

        let dx = x2 - x1;
        let dy = y2 - y1;

        let mut dir_x = 1;
        let mut dir_y = 1;

        // update direction to handle negatives
        if dx < 0 {
            dir_x = -1;
        }
        if dy < 0 {
            dir_y = -1;
        }

        if dx == 0 && dy == 0 {
            break 'outer;
        }

        count += 1;

        // step along to bring difference between dx and dy to 0
        if (dy * dir_y) > (dx * dir_x) {
            next.y += dir_y;
        } else if dx != 0 {
            next.x += dir_x;
        }
    }

    count
}

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let space = expand_space(input).expect("should expand space");
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
                        x: x as i32,
                        y: y as i32,
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

    paths.iter().for_each(|el| {
        let id1 = el.0.id;
        let id2 = el.1.id;
        if id1 == 5 && id2 == 9 {
            min_path_length(el.0.pos, el.1.pos);
        }
    });

    let path_sum: u32 = paths
        .iter()
        .map(|(a, b)| min_path_length(a.pos, b.pos))
        .sum();

    Ok(path_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_space() {
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

        let expected = "\
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

        assert_eq!(
            expected,
            expand_space(input).expect("should return expected value")
        );
    }

    #[test]
    fn example_input() {
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

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
