/*
  Day 6 part 1 looks very easy to solve with a brute force approach.

  But it yells for "MATHS", so I'll try to do it with maths instead.

  The equation appears to be:
  distance = (total_time - time_pressed) * time_pressed

  We need to compute time_pressed that provide higher results than a given distance.
  Using an old TI-89 with the magic solve(),
  the two magic solving equations to compute the time_pressed appear to be:
  (total_time - sqrt(total_time^2 - 4 * distance)) / 2
  and
  (total_time + sqrt(total_time^2 - 4 * distance)) / 2

  We can floor the approximation of the first one, and ceil the second one,
  and add some epsilon to work with the range we need to compute.
  Found the right combination of ceiling, flooring, adding or substrating epsilon
  by trial and error, of course.

  Then the number of higher results between the two is second-first-1.
*/

use nom::{
    self,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::opt,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn parse_input_data_part_1(data: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    map(
        tuple((
            tag("Time:"),
            space1,
            separated_list1(space1, nom::character::complete::u64),
            line_ending,
            tag("Distance:"),
            space1,
            separated_list1(space1, nom::character::complete::u64),
            opt(line_ending),
        )),
        |(_, _, times, _, _, _, distances, _)| (times, distances),
    )(data)
}

#[inline]
fn compute_many_ways_to_win(time: u64, distance: u64) -> u64 {
    let time = time as f64;
    let distance = distance as f64;
    let common_sqrt = (time * time - 4_f64 * distance).sqrt();
    let first = ((time - common_sqrt) / 2_f64 + f64::EPSILON).floor() as u64;
    let second = ((time + common_sqrt) / 2_f64 - f64::EPSILON).ceil() as u64;
    return second - first - 1;
}

pub fn day_6_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data_part_1(data).expect("Unable to parse input data");
    let (times, distances) = data;
    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| compute_many_ways_to_win(time, distance))
        .fold(1, |acc, x| acc * x) as i64
}

fn parse_number_with_random_spaces(input: &str) -> IResult<&str, u64> {
    map_res(separated_list1(space1, digit1), |vec: Vec<&str>| {
        vec.concat().parse::<u64>()
    })(input)
}

fn parse_input_data_part_2(data: &str) -> IResult<&str, (u64, u64)> {
    map(
        tuple((
            tag("Time:"),
            space1,
            parse_number_with_random_spaces,
            line_ending,
            tag("Distance:"),
            space1,
            parse_number_with_random_spaces,
            opt(line_ending),
        )),
        |(_, _, time, _, _, _, distance, _)| (time, distance),
    )(data)
}

pub fn day_6_part_2(data: &str) -> i64 {
    let (_, (time, distance)) = parse_input_data_part_2(data).expect("Unable to parse input data");

    compute_many_ways_to_win(time, distance) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_day_6_part_1() {
        assert_eq!(day_6_part_1(EXAMPLE), 288);
    }

    #[test]
    fn test_day_6_part_2() {
        assert_eq!(day_6_part_2(EXAMPLE), 71503);
    }
}
