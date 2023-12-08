use num::integer::lcm;
use std::collections::{HashMap, VecDeque};
use std::io::Result;

const START_SUFFIX: char = 'A';
const END_SUFFIX: char = 'Z';

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum NodeKind {
    Start,
    End,
    Regular,
}

#[derive(Debug, Clone, Hash)]
pub struct Node {
    value: String,
    left: String,
    right: String,
    kind: NodeKind,
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
            kind: match value.chars().last() {
                Some(START_SUFFIX) => NodeKind::Start,
                Some(END_SUFFIX) => NodeKind::End,
                _ => NodeKind::Regular,
            },
        }
    }
}

///
/// Hanted Wasteland
///
/// - follow (L)eft, (R)ight instructions to navigate
/// - if instruction set doesn't terminate, repeat from start until termination
///
/// Part 2
///
/// - now there's ghosts
/// - starts end with 'A'
/// - ends end with 'Z'
///
pub fn run(input: &str) -> Result<String> {
    let mut lines = input.lines().filter(|line| !line.is_empty());
    let original_moves: VecDeque<char> = lines
        .next()
        .expect("should have moves line")
        .chars()
        .collect();

    let mut starts: Vec<Node> = vec![];
    let mut nodes: HashMap<String, Node> = HashMap::new();

    // add node records
    lines.for_each(|line| {
        let node = Node::from(line);
        nodes.insert(node.value.clone(), node.clone());
        if node.kind == NodeKind::Start {
            starts.push(node.clone());
        }
    });

    let mut step_count = 0;
    let mut current_nodes: Vec<Node> = starts.clone();
    let mut moves = original_moves.clone();
    let mut end_depths: HashMap<String, u64> = HashMap::new();

    // loop until no more moves
    'outer: while !moves.is_empty() {
        step_count += 1;
        let dir = moves.pop_front().unwrap();
        let mut next_nodes: Vec<Node> = vec![];

        current_nodes.iter().for_each(|node| {
            let node = node.clone();

            // find next nodes and update depth counters
            match dir {
                'L' => {
                    let next_node = nodes.get(&node.left).unwrap().clone();
                    let record = end_depths.entry(next_node.value.clone()).or_insert(0);
                    *record = step_count;

                    // add to next step if not an end node
                    if next_node.kind != NodeKind::End {
                        next_nodes.push(next_node);
                    }
                }
                _ => {
                    let next_node = nodes.get(&node.right).unwrap().clone();
                    let record = end_depths.entry(next_node.value.clone()).or_insert(0);
                    *record = step_count;

                    // add to next step if not an end node
                    if next_node.kind != NodeKind::End {
                        next_nodes.push(next_node);
                    }
                }
            };
        });

        // if all ends found at once, stop
        if next_nodes.iter().all(|node| node.kind == NodeKind::End) {
            break 'outer;
        }

        // repeat original move set
        if moves.is_empty() {
            moves = original_moves.clone();
        }

        // update nodes to check
        current_nodes = next_nodes.clone();
    }

    // loop over, find least common multiple of end depths
    let depths_lcm: u64 = end_depths
        .iter()
        .filter(|(step, _)| step.ends_with('Z'))
        .map(|(_, &depth)| depth)
        .reduce(lcm)
        .unwrap();

    Ok(depths_lcm.to_string())
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

    #[test]
    fn example_input_ghosts() {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let expected = "6".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
