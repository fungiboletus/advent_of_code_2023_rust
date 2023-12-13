/*
    Day 12 is one of those days that I know I would spend too much
    time trying to solve it in a bad way, so I looked at a good and
    elegant solution online.

    The way I would have done part 1 is by bruteforcing with a cache.

    This solution is from https://github.com/fuglede/adventofcode/blob/master/2023/day12/solutions.py
    Thanks @fuglede for sharing it!

    It kind of bruteforce it with a cache, but it's well done
    and has a small trick to simplify the solution by adding a operational condition
    at the end of each spring.

    I realise that it wasn't too hard. But I don't like this kind of problems.

    To fully be lazy, ChatGPT is relatively good at converting good Python solutions to Rust,
    but it had to get some help to fully manage.

    To my defense, I did optimise the solution by doing the caching
    using hashes of the pointers. The initial version was using two Vec as keys and
    was copying a lot of data.

    This did involve some more advanced Rust code than I'm used to. The Phantom data thing with
    lifetimes is a bit weird for example.
*/

// Let's make it faster by using rayon and computing each spring in parallel.
use rayon::prelude::*;

use core::hash::Hash;
use std::{collections::HashMap, hash::Hasher, marker::PhantomData};

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of, space1, u64},
    combinator::map,
    multi::{many0, separated_list0},
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}
impl SpringCondition {
    fn from_char(c: char) -> SpringCondition {
        match c {
            '.' => SpringCondition::Operational,
            '#' => SpringCondition::Damaged,
            '?' => SpringCondition::Unknown,
            _ => panic!("Unknown condition: {}", c),
        }
    }
}

fn parse_spring(data: &str) -> IResult<&str, (Vec<SpringCondition>, Vec<u64>)> {
    map(
        tuple((many0(one_of(".#?")), space1, separated_list0(tag(","), u64))),
        |(solution, _, sizes)| {
            (
                solution
                    .into_iter()
                    .map(|c| SpringCondition::from_char(c))
                    .collect(),
                sizes,
            )
        },
    )(data)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<(Vec<SpringCondition>, Vec<u64>)>> {
    separated_list0(line_ending, parse_spring)(data)
}

// This MemoKey uses pointers
// to avoid copying the slices when doing the memoisation.
struct MemoKey<'a> {
    spring: *const SpringCondition, // Pointer to spring slice
    sizes: *const u64,              // Pointer to sizes slice
    num_done_in_group: u64,
    _marker: PhantomData<&'a ()>, // Indicates the struct is associated with lifetime 'a
}
impl<'a> Hash for MemoKey<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.spring.hash(state);
        self.sizes.hash(state);
        self.num_done_in_group.hash(state);
    }
}
impl<'a> PartialEq for MemoKey<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.spring == other.spring
            && self.sizes == other.sizes
            && self.num_done_in_group == other.num_done_in_group
    }
}

impl<'a> Eq for MemoKey<'a> {}

fn num_solutions<'a>(
    spring: &'a [SpringCondition],
    sizes: &'a [u64],
    num_done_in_group: u64,
    memo: &mut HashMap<MemoKey<'a>, u64>,
) -> u64 {
    if spring.is_empty() {
        return (sizes.is_empty() && num_done_in_group == 0) as u64;
    }

    let key = MemoKey {
        spring: spring.as_ptr(),
        sizes: sizes.as_ptr(),
        num_done_in_group,
        _marker: PhantomData,
    };
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let mut num_sols = 0;
    let first_condition = spring[0];

    if first_condition == SpringCondition::Damaged || first_condition == SpringCondition::Unknown {
        num_sols += num_solutions(&spring[1..], sizes, num_done_in_group + 1, memo);
    }

    if first_condition == SpringCondition::Operational
        || first_condition == SpringCondition::Unknown
    {
        if num_done_in_group > 0 {
            if let Some((&first_size, rest_sizes)) = sizes.split_first() {
                if first_size == num_done_in_group {
                    num_sols += num_solutions(&spring[1..], rest_sizes, 0, memo);
                }
            }
        } else {
            num_sols += num_solutions(&spring[1..], sizes, 0, memo);
        }
    }

    memo.insert(key, num_sols);
    num_sols
}

pub fn day_12_part_1(data: &str) -> i64 {
    let (_, springs) = parse_input_data(data).expect("Failed to parse input data");

    springs
        .par_iter()
        .map(|(spring, sizes)| {
            let mut extended_spring = Vec::with_capacity(spring.len() + 1);
            extended_spring.extend(spring.iter());
            extended_spring.push(SpringCondition::Operational);
            let mut memo = HashMap::new();
            num_solutions(&extended_spring, sizes, 0, &mut memo)
        })
        .sum::<u64>() as i64
}

pub fn day_12_part_2(data: &str) -> i64 {
    let (_, springs) = parse_input_data(data).expect("Failed to parse input data");

    springs
        .par_iter()
        .map(|(spring, sizes)| {
            let mut extended_spring: Vec<SpringCondition> =
                Vec::with_capacity(spring.len() * 5 + 5);
            let mut extended_sizes: Vec<u64> = Vec::with_capacity(sizes.len() * 5);
            for i in 0..5 {
                extended_spring.extend(spring.iter());
                extended_spring.push(if i < 4 {
                    SpringCondition::Unknown
                } else {
                    SpringCondition::Operational
                });
                extended_sizes.extend(sizes.iter());
            }

            let mut memo = HashMap::new();
            num_solutions(&extended_spring, &extended_sizes, 0, &mut memo)
        })
        .sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_day_12_part_1() {
        assert_eq!(day_12_part_1(EXAMPLE), 21);
    }

    #[test]
    fn test_day_12_part_2() {
        assert_eq!(day_12_part_2(EXAMPLE), 525152);
    }
}
