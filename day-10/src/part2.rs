use std::{cell::RefCell, collections::HashSet, error::Error};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum Direction {
    Down,
    Left,
    Up,
    Right,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pipe {
    char: char,
    origin: Point,
    ends: Vec<Direction>,
}

impl Pipe {
    pub fn from(x: usize, y: usize, char: char) -> Pipe {
        use Direction::*;

        Pipe {
            char,
            origin: Point { x, y },
            ends: match char {
                '|' => vec![Up, Down],
                '-' => vec![Left, Right],
                'L' => vec![Up, Right],
                'J' => vec![Up, Left],
                '7' => vec![Left, Down],
                'F' => vec![Right, Down],
                'S' => vec![Up, Down, Left, Right],
                '.' => vec![],
                _ => vec![],
            },
        }
    }
}

#[derive(Debug)]
pub struct Maze {
    pipes: Vec<Pipe>,
    start: Pipe,
    width: usize,
    path: RefCell<Vec<Pipe>>,
    start_override: RefCell<char>,
}

impl Maze {
    pub fn from(pipes: Vec<Pipe>, width: usize) -> Maze {
        Maze {
            width,
            start: pipes
                .iter()
                .find(|pipe| pipe.ends.len() == 4)
                .expect("should have a start")
                .clone(),
            pipes,
            path: RefCell::from(Vec::new()),
            start_override: '-'.into(),
        }
    }

    pub fn next_pipe(&self, direction: &Direction, origin: Point) -> Option<&Pipe> {
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
            return self.pipes.get(next_index as usize);
        }

        None
    }

    pub fn search(&self, pipe: &Pipe, from: Direction) -> u32 {
        use Direction::*;

        let mut next_pipes: Vec<(&Pipe, Direction, u32)> = vec![(pipe, from, 0)];

        let mut steps: Vec<u32> = vec![];
        let mut loop_count: u32 = 0;

        'outer: loop {
            if next_pipes.is_empty() {
                break 'outer;
            }

            if loop_count == 1 {
                let mut directions: Vec<Direction> = next_pipes.iter().map(|el| el.1).collect();
                directions.sort();

                // figure out how to treat the start character `S` for interior checks
                *self.start_override.borrow_mut() = match &directions[..] {
                    [Left, Up] => 'F',
                    [Up, Right] => '7',
                    [Down, Left] => 'L',
                    [Down, Right] => 'J',
                    _ => '-',
                };

                // limit search to one direction
                next_pipes = vec![*next_pipes.iter().collect::<Vec<_>>()[0]];
            }

            // add to path
            next_pipes.iter().for_each(|el| {
                self.path.borrow_mut().push(el.0.clone());
            });

            loop_count += 1;

            // check and rebuild next_pipes Vec
            next_pipes = next_pipes
                .iter()
                // discard any `None`s
                .flat_map(|(pipe, dir, step)| {
                    // add check step
                    steps.push(*step);
                    // return Vec of next pipes to check
                    pipe.ends
                        .iter()
                        // filter out directions that would backtrack
                        .filter(|&end| *end != *dir)
                        .flat_map(|end| {
                            if let Some(next_pipe) = self.next_pipe(end, pipe.origin) {
                                if next_pipe.ends.len() < 4 {
                                    // ignore start pipe
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

        self.find_interiors()
    }

    /// Counts interiors by scanning across, keeping track of the number
    /// of vertical edges crossed by counting relevant characters
    /// (e.g. `|`, `F`, `7`, or `|`, `L`, `J`).
    ///
    /// A character counts as an interior if encountered after an odd number
    /// of vertical edges.
    pub fn find_interiors(&self) -> u32 {
        let mut count = 0;
        let mut num_edges: u32 = 0;
        let mut verts = vec!['|', 'F', '7'];
        let start_char = self.start_override.borrow();
        let path = self.path.borrow();
        let path_set: HashSet<&Pipe> = HashSet::from_iter(path.iter());

        // include `S` only if it counts as one of the selected corners
        if verts.contains(&start_char) {
            verts.push(self.start.char);
        }

        for (i, pipe) in self.pipes.iter().enumerate() {
            if path_set.contains(pipe) {
                if verts.contains(&pipe.char) {
                    num_edges += 1;
                }
            } else if num_edges % 2 != 0 {
                count += 1;
            }
            if (i + 1) % self.width == 0 {
                num_edges = 0;
            }
        }

        count
    }
}

///
/// - find and build path
///
pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let lines = input.lines();
    let width = lines.clone().peekable().peek().unwrap().len();

    // convert all symbols to Pipe structs
    let pipes: Vec<Pipe> = lines
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
    fn example_input_internal_small() {
        let input = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let expected = "4".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }

    #[test]
    fn example_input_internal_large() {
        let input = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let expected = "8".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }

    #[test]
    fn example_input_internal_large_2() {
        let input = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let expected = "10".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
