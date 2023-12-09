use num::integer::lcm;
use std::collections::HashMap;
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
    let moves: Vec<char> = lines
        .next()
        .expect("should have moves line")
        .chars()
        .collect();

    let mut nodes: HashMap<String, Node> = HashMap::new();

    // add node records
    let mut check_nodes: Vec<Node> = lines.fold(vec![], |mut acc, line| {
        let node = Node::from(line);

        if node.kind == NodeKind::Start {
            acc.push(node.clone());
        }

        nodes.insert(node.value.to_string(), node);

        acc
    });

    let mut step_count: u64 = 0;
    let mut end_depths: Vec<u64> = vec![];

    // loop until no more moves
    'outer: loop {
        // nothing left to check
        if check_nodes.is_empty() {
            break 'outer;
        }

        let i = (step_count % moves.len() as u64) as usize;
        let dir = moves.get(i).unwrap();
        step_count += 1;

        let mut next_nodes: Vec<Node> = vec![];

        check_nodes.iter().for_each(|node| {
            // find next nodes and update depth counters
            let next_node = match dir {
                'L' => nodes.get(&node.left).unwrap(),
                _ => nodes.get(&node.right).unwrap(),
            };

            if next_node.kind == NodeKind::End {
                // record depth for end node
                end_depths.push(step_count);
            } else {
                // add to next step if not an end node
                next_nodes.push(next_node.clone())
            }
        });

        // nothing left to check
        if next_nodes.is_empty() {
            break 'outer;
        }

        check_nodes = next_nodes;
    }

    // loop over, find least common multiple of end depths
    let depths_lcm = end_depths.into_iter().reduce(lcm).unwrap();

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
