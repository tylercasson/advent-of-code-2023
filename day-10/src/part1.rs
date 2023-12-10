use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Start,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Pipe {
    origin: Point,
    ends: Vec<Direction>,
}

impl Pipe {
    pub fn from(x: usize, y: usize, char: char) -> Option<Pipe> {
        use Direction::*;
        if char == '.' {
            return None;
        }
        Some(Pipe {
            origin: Point { x, y },
            ends: match char {
                '|' => vec![Up, Down],
                '-' => vec![Left, Right],
                'L' => vec![Up, Right],
                'J' => vec![Up, Left],
                '7' => vec![Left, Down],
                'F' => vec![Right, Down],
                'S' => vec![Up, Down, Left, Right],
                _ => vec![],
            },
        })
    }
}

#[derive(Debug)]
pub struct Maze {
    pipes: Vec<Option<Pipe>>,
    start: Pipe,
    width: usize,
}

impl Maze {
    pub fn from(pipes: Vec<Option<Pipe>>, width: usize) -> Maze {
        Maze {
            width,
            start: pipes
                .iter()
                .enumerate()
                .find(|(_, pipe)| {
                    if let Some(pipe) = pipe {
                        pipe.ends.len() == 4
                    } else {
                        false
                    }
                })
                .map(|(_, pipe)| pipe.clone().expect("should be valid pipe"))
                .expect("should have a start"),
            pipes,
        }
    }

    pub fn next_pipe(&self, direction: &Direction, origin: Point) -> &Option<Pipe> {
        use Direction::*;

        let index = origin.x + origin.y * self.width;

        let next_index = match direction {
            Up => index as isize - self.width as isize,
            Down => index as isize + self.width as isize,
            Left => index as isize - 1,
            Right => index as isize + 1,
            _ => index as isize,
        };

        if next_index != index as isize && next_index >= 0 && next_index < self.pipes.len() as isize
        {
            return self.pipes.get(next_index as usize).unwrap();
        }

        &None
    }

    pub fn search(&self, pipe: &Pipe, from: Direction) -> u32 {
        use Direction::*;

        let mut next_pipes: Vec<Option<(&Pipe, Direction, u32)>> =
            vec![Some((pipe, from.clone(), 0))];

        let mut steps: Vec<u32> = vec![];

        'outer: loop {
            if next_pipes.is_empty() {
                break 'outer;
            }

            // check and rebuild next_pipes Vec
            next_pipes = next_pipes
                .iter()
                // discard any `None`s
                .flatten()
                .flat_map(|(pipe, dir, step)| {
                    // add check step
                    steps.push(*step);
                    // return Vec of next pipes to check
                    pipe.ends
                        .iter()
                        // filter out directions that would backtrack
                        .filter(|&end| *end != *dir)
                        .map(|end| {
                            if let Some(next_pipe) = self.next_pipe(end, pipe.origin) {
                                // ignore start pipe
                                if next_pipe.ends.len() < 4 {
                                    // check whether the next pipe is compatible
                                    match end {
                                        Up => {
                                            if next_pipe.ends.contains(&Down) {
                                                Some((next_pipe, Down, *step + 1))
                                            } else {
                                                None
                                            }
                                        }
                                        Down => {
                                            if next_pipe.ends.contains(&Up) {
                                                Some((next_pipe, Up, *step + 1))
                                            } else {
                                                None
                                            }
                                        }
                                        Left => {
                                            if next_pipe.ends.contains(&Right) {
                                                Some((next_pipe, Right, *step + 1))
                                            } else {
                                                None
                                            }
                                        }
                                        Right => {
                                            if next_pipe.ends.contains(&Left) {
                                                Some((next_pipe, Left, *step + 1))
                                            } else {
                                                None
                                            }
                                        }
                                        _ => None,
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }

        steps.iter().max().unwrap() / 2 + 1
    }
}

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let lines = input.lines();
    let width = lines.clone().peekable().peek().unwrap().len();

    // convert all symbols to Pipe structs
    let pipes: Vec<Option<Pipe>> = lines
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| Pipe::from(x, y, char))
        })
        .collect();

    let maze = Maze::from(pipes, width);
    let steps = maze.search(&maze.start.clone(), Direction::Start);

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example_input_short() {
        let input = "\
.....
.S-7.
.|.|.
.L-J.
.....";

        let expected = "4".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }

    #[test]
    fn example_input_long() {
        let input = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let expected = "8".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
