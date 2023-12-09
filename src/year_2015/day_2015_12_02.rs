/*
    Comments.
*/

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u64},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Box {
    length: u64,
    width: u64,
    height: u64,
}

fn parse_box_dimensions(data: &str) -> IResult<&str, Box> {
    map(
        tuple((u64, tag("x"), u64, tag("x"), u64)),
        |(length, _, width, _, height)| Box {
            length,
            width,
            height,
        },
    )(data)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<Box>> {
    separated_list0(line_ending, parse_box_dimensions)(data)
}

pub fn day_2015_12_02_part_1(data: &str) -> i64 {
    let (_, boxes) = parse_input_data(data).unwrap();

    boxes
        .iter()
        .map(|boxe| {
            let side_a = boxe.length * boxe.width;
            let side_b = boxe.width * boxe.height;
            let side_c = boxe.height * boxe.length;
            let smallest_side = side_a.min(side_b).min(side_c);
            2 * side_a + 2 * side_b + 2 * side_c + smallest_side
        })
        .sum::<u64>() as i64
}

pub fn day_2015_12_02_part_2(data: &str) -> i64 {
    let (_, boxes) = parse_input_data(data).unwrap();

    boxes
        .iter()
        .map(|boxe| {
            let Box {
                length,
                width,
                height,
            } = boxe;
            let smallest_wrap = (length + length + width + width)
                .min(length + length + height + height)
                .min(width + width + height + height);
            let ribbon_length = length * width * height;
            smallest_wrap + ribbon_length
        })
        .sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2015_12_02_part_1() {
        assert_eq!(day_2015_12_02_part_1("2x3x4"), 58);
        assert_eq!(day_2015_12_02_part_1("1x1x10"), 43);
        assert_eq!(day_2015_12_02_part_1("2x3x4\n1x1x10"), 101);
    }

    #[test]
    fn test_day_2015_12_02_part_2() {
        assert_eq!(day_2015_12_02_part_2("2x3x4"), 34);
        assert_eq!(day_2015_12_02_part_2("1x1x10"), 14);
        assert_eq!(day_2015_12_02_part_2("2x3x4\n1x1x10"), 48);
    }
}
