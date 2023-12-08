/*
    Day 8 looks like a simple path finding problem. First part is relatively easy.

    Second part will potentially require a Djikstra's algorithm.

    The 3 characters letters will be converted to a number because it's going to be faster.
*/

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of, satisfy},
    combinator::{map, map_res},
    multi::{count, many_till, separated_list0},
    sequence::{preceded, tuple},
    IResult,
};

type NodeName = u16;

fn letters_to_number(letters: [char; 3]) -> NodeName {
    let mut number = 0;
    for letter in letters.iter() {
        number <<= 5;
        number |= *letter as u16 - 65;
    }
    number
}

// For debugging purposes
#[allow(dead_code)]
fn number_to_letters(number: NodeName) -> [char; 3] {
    let mut letters = ['A'; 3];
    let mut number = number;
    for letter in letters.iter_mut().rev() {
        *letter = ((number & 31) as u8 + 65) as char;
        number >>= 5;
    }
    letters
}

fn parse_node_name(data: &str) -> IResult<&str, NodeName> {
    map(
        count(satisfy(|c: char| c.is_ascii_uppercase()), 3),
        |letters: Vec<char>| letters_to_number([letters[0], letters[1], letters[2]]),
    )(data)
}

#[derive(Debug)]
struct Node {
    name: NodeName,
    left: NodeName,
    right: NodeName,
}

fn parse_node(data: &str) -> IResult<&str, Node> {
    map(
        tuple((
            parse_node_name,
            preceded(tag(" = ("), parse_node_name),
            preceded(tag(", "), parse_node_name),
            tag(")"),
        )),
        |(name, left, right, _)| Node { name, left, right },
    )(data)
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parse_directions(data: &str) -> IResult<&str, Vec<Direction>> {
    map_res(many_till(one_of("LR"), line_ending), |(directions, _)| {
        directions
            .iter()
            .map(|direction| match direction {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                _ => return Err(nom::Err::Failure((data, nom::error::ErrorKind::Char))),
            })
            .collect()
    })(data)
}

fn parse_input_data(data: &str) -> IResult<&str, (Vec<Direction>, Vec<Node>)> {
    map(
        tuple((
            parse_directions,
            line_ending,
            separated_list0(line_ending, parse_node),
        )),
        |(directions, _, nodes)| (directions, nodes),
    )(data)
}

const FIRST_NODE: NodeName = 0;
const LAST_NODE: NodeName = 26425;
const MAX_NODES: usize = 26426;

// We build a dictionary, key is the nodename, and values is left and right.
// It's an array. We waste a bit some ram and cpu caches but I'm slightly lazy.
// And it's fast.
fn build_dictionary(nodes: &[Node]) -> [(NodeName, NodeName); MAX_NODES] {
    let mut dictionary: [(NodeName, NodeName); MAX_NODES] = [(FIRST_NODE, FIRST_NODE); MAX_NODES];
    for node in nodes.iter() {
        dictionary[node.name as usize] = (node.left, node.right);
    }
    dictionary
}

fn compute_number_of_iterations(
    dictionary: &[(NodeName, NodeName); MAX_NODES],
    directions: &[Direction],
    start: NodeName,
    ending_nodes: &[bool; MAX_NODES],
) -> i64 {
    let mut current_node = start;
    let mut nb_iterations = 0_i64;
    let mut directions_index = 0_usize;
    let directions_len = directions.len();
    loop {
        nb_iterations += 1;
        let direction = &directions[directions_index];

        // Loop on the directions
        directions_index = (directions_index + 1) % directions_len;

        let (left, right) = dictionary[current_node as usize];
        current_node = match direction {
            Direction::Left => left,
            Direction::Right => right,
        };
        if ending_nodes[current_node as usize] {
            break;
        }
    }

    nb_iterations
}

pub fn day_8_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    let (directions, nodes) = data;
    let dictionary = build_dictionary(&nodes);

    // Build a dictionary of ending nodes
    // It's done like this to reuse code with part 2
    let mut ending_nodes: [bool; MAX_NODES] = [false; MAX_NODES];
    ending_nodes[LAST_NODE as usize] = true;

    compute_number_of_iterations(&dictionary, &directions, FIRST_NODE, &ending_nodes)
}

/**
 *
 * After all, part 2 is not a Djikstra's algorithm.
 *
 * Naive solution is too slow, so we need to find a way to optimize it.
 *
 * I didn't remember the gcd algorithm so I had to look it up, but otherwise so far so good.
 *
 * After implementing half of the solution, I thought that it wasn't going to work
 * but the problem was designed to work with this solution after all.
 * I found out on the subreddit that it was the way to go.
 */
#[inline]
fn node_ends_with_a(node: NodeName) -> bool {
    return node & 31 == 0;
}

#[inline]
fn node_ends_with_z(node: NodeName) -> bool {
    return node & 31 == 25;
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline]
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_list(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .cloned()
        .reduce(|a, b| lcm(a, b))
        .unwrap_or(1)
}

pub fn day_8_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    let (directions, nodes) = data;
    let dictionary = build_dictionary(&nodes);

    // find out all the starting points
    let current_nodes: Vec<NodeName> = nodes
        .iter()
        .filter(|node| node_ends_with_a(node.name))
        .map(|node| node.name)
        .collect();

    // Build a dictionary of ending nodes
    let mut ending_nodes: [bool; MAX_NODES] = [false; MAX_NODES];
    for node in nodes.iter() {
        if node_ends_with_z(node.name) {
            ending_nodes[node.name as usize] = true;
        }
    }

    // Compute all the number of iterations required
    let nb_iterations_for_all = current_nodes
        .iter()
        .map(|node| compute_number_of_iterations(&dictionary, &directions, *node, &ending_nodes))
        .collect::<Vec<i64>>();

    // Compute the least common multiple
    lcm_list(
        &nb_iterations_for_all
            .iter()
            .map(|nb| *nb as u64)
            .collect::<Vec<u64>>(),
    ) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_EXAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const SECOND_EXAMPLE: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    // Adapted example because they somehow used digits.
    const PART_2_EXAMPLE: &str = "LR

DDA = (DDB, XXX)
DDB = (XXX, DDZ)
DDZ = (DDB, XXX)
EEA = (EEB, XXX)
EEB = (EEC, EEC)
EEC = (EEZ, EEZ)
EEZ = (EEB, EEB)
XXX = (XXX, XXX)";

    #[test]
    fn test_numbers_letters_conversions() {
        assert_eq!(letters_to_number(['A', 'A', 'A']), 0);
        assert_eq!(
            number_to_letters(letters_to_number(['Z', 'Z', 'Z'])),
            ['Z', 'Z', 'Z']
        );
        assert_eq!(
            number_to_letters(letters_to_number(['M', 'H', 'A'])),
            ['M', 'H', 'A']
        );
        assert_eq!(
            number_to_letters(letters_to_number(['V', 'Q', 'D'])),
            ['V', 'Q', 'D']
        );
    }

    #[test]
    fn test_day_8_part_1() {
        assert_eq!(day_8_part_1(FIRST_EXAMPLE), 2);
        assert_eq!(day_8_part_1(SECOND_EXAMPLE), 6);
    }

    #[test]
    fn test_node_ends_with_a() {
        assert!(node_ends_with_a(0));
        assert!(!node_ends_with_a(1));
        assert!(node_ends_with_a(letters_to_number(['M', 'H', 'A'])));
        assert!(!node_ends_with_a(letters_to_number(['V', 'Q', 'D'])));
    }

    #[test]
    fn test_node_ends_with_z() {
        assert!(node_ends_with_z(25));
        assert!(!node_ends_with_z(FIRST_NODE));
        assert!(node_ends_with_z(LAST_NODE));
        assert!(node_ends_with_z(letters_to_number(['V', 'Q', 'Z'])));
        assert!(!node_ends_with_z(letters_to_number(['V', 'Q', 'D'])));
        assert!(node_ends_with_z(letters_to_number(['B', 'Z', 'Z'])));
    }

    #[test]
    fn test_day_8_part_2() {
        assert_eq!(day_8_part_2(PART_2_EXAMPLE), 6);
    }
}
