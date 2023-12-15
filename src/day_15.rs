/*
    Comments.
*/

use std::rc::Rc;

use nom::{
    branch::alt,
    character::{
        complete::{char, none_of, satisfy},
        is_digit,
    },
    combinator::map,
    multi::{many0, separated_list0},
    sequence::tuple,
    IResult,
};

fn parse_input_data_part_1(data: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list0(char(','), many0(none_of(",")))(data)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum OperationType {
    Remove,
    AddOrReplace,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Lens {
    name: String,
    hash: u8,
    focal: Option<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Operation {
    operation_type: OperationType,
    lens: Lens,
}

#[inline]
fn compute_hash(text: &Vec<char>) -> u8 {
    // reminder of divide by 256, using a bit mask
    //.fold(0_u64, |hash, c| ((hash + (*c as u64)) * 17) % 256)
    text.iter()
        .fold(0_u64, |hash, c| ((hash + (*c as u64)) * 17) & 0xFF) as u8
}

fn parse_input_data_part_2(data: &str) -> IResult<&str, Vec<Operation>> {
    separated_list0(
        char(','),
        map(
            tuple((
                many0(none_of("-=,")),
                alt((
                    map(char('-'), |_| (OperationType::Remove, None)),
                    tuple((
                        map(char('='), |_| OperationType::AddOrReplace),
                        map(satisfy(|c| is_digit(c as u8)), |c| Some(c as u8 - 48)),
                    )),
                )),
            )),
            |(name, (operation_type, focal))| {
                let hash = compute_hash(&name);
                let name = name.iter().collect::<String>();
                Operation {
                    operation_type,
                    lens: Lens { name, hash, focal },
                }
            },
        ),
    )(data)
}

pub fn day_15_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data_part_1(data).expect("Failed to parse input data");

    data.iter()
        .map(|string| compute_hash(string) as i64)
        .sum::<i64>()
}

pub fn day_15_part_2(data: &str) -> i64 {
    let (_, operations) = parse_input_data_part_2(data).expect("Failed to parse input data");

    // 256 boxes as an array containing Vec. gave a try with LinkedList but this is not stable.
    const INIT_BOX: Vec<Rc<Lens>> = Vec::new();
    let mut boxes: [Vec<Rc<Lens>>; 256] = [INIT_BOX; 256];

    for operation in operations {
        //println!("{:?}", operation);
        let hash = operation.lens.hash as usize;
        match operation.operation_type {
            OperationType::Remove => {
                // remove the lens from the box
                let new_box = boxes[hash]
                    .iter()
                    .filter(|lens| lens.name != operation.lens.name)
                    .cloned()
                    .collect::<Vec<Rc<Lens>>>();
                boxes[hash] = new_box;
            }
            OperationType::AddOrReplace => {
                // If we need to replace the lens
                if let Some((index, _)) = boxes[hash]
                    .iter()
                    .enumerate()
                    .find(|(_, lens)| lens.name == operation.lens.name)
                {
                    boxes[hash][index] = Rc::new(operation.lens);
                // Or add it
                } else {
                    boxes[hash].push(Rc::new(operation.lens));
                }
            }
        }
        /*println!(
            "{:?}",
            boxes
                .iter()
                .filter(|lens| lens.len() > 0)
                .collect::<Vec<&Vec<Rc<Lens>>>>()
        );*/
    }
    /*println!("{:?}", left);
    println!("{:?}", boxes);*/

    boxes
        .iter()
        .enumerate()
        .map(|(i, box_)| {
            box_.iter()
                .enumerate()
                .map(|(j, lens)| {
                    let focal = lens.focal.expect("No focal: weird");
                    (i + 1) as i64 * (j + 1) as i64 * focal as i64
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_day_15_part_1() {
        assert_eq!(day_15_part_1(EXAMPLE), 1320);
    }

    #[test]
    fn test_day_15_part_2() {
        assert_eq!(day_15_part_2(EXAMPLE), 145);
    }
}
