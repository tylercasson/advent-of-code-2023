use std::collections::{HashMap, VecDeque};
use std::io::Result;

const START_VALUE: &str = "AAA";
const END_VALUE: &str = "ZZZ";

#[derive(Debug)]
pub struct Node {
    value: String,
    left: String,
    right: String,
}

impl Node {
    pub fn from(line: &str) -> Node {
        let (value, rest) = line.split_once(" = ").unwrap();
        let rest = rest.replace(['(', ')'], "");
        let (left, right) = rest.split_once(", ").unwrap();

        Node {
            value: value.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

///
/// Hanted Wasteland
///
/// - follow (L)eft, (R)ight instructions to navigate
/// - if instruction set doesn't terminate, repeat from start until termination
/// -
///
/// - later: find shortcuts through nodes?
///
pub fn run(input: &str) -> Result<String> {
    let mut lines = input.lines().filter(|line| !line.is_empty());
    let original_moves: VecDeque<char> = lines
        .next()
        .expect("should have moves line")
        .chars()
        .collect();

    let mut map: HashMap<String, Node> = HashMap::new();

    // add node records
    lines.for_each(|line| {
        let node = Node::from(line);
        map.insert(node.value.clone(), node);
    });

    // get starting node
    let mut node = map.get(START_VALUE).expect("should have starting node");

    let mut step_count = 0;

    let mut moves = original_moves.clone();
    'outer: while !moves.is_empty() {
        let dir = moves.pop_front().unwrap();
        match dir {
            'L' => node = map.get(&node.left).unwrap(),
            _ => node = map.get(&node.right).unwrap(),
        };
        step_count += 1;
        if node.value == END_VALUE {
            break 'outer;
        } else if moves.is_empty() {
            moves = original_moves.clone();
        }
    }

    Ok(step_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example_input_no_repeats() {
        let input = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let expected = "2".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }

    #[test]
    fn example_input_repeats() {
        let input = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let expected = "6".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
