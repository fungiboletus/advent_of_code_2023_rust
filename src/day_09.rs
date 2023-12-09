/*
    Should be relatively straightforward, so I suspect I'm missing something.
*/

use nom::{
    character::complete::{i64, line_ending, space1},
    multi::{separated_list0, separated_list1},
    IResult,
};

fn parse_history(data: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, i64)(data)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list0(line_ending, parse_history)(data)
}

fn solve_history_part_1(history: &Vec<i64>) -> i64 {
    let mut workbench = history.clone();
    let mut last_index = workbench.len() - 1;

    let mut last_numbers_per_turn = Vec::new();

    loop {
        let mut all_zeros = true;
        last_numbers_per_turn.push(workbench[last_index]);

        // compute the difference between each element and the next one
        for i in 0..last_index {
            let diff = workbench[i + 1] - workbench[i];
            workbench[i] = diff;
            if diff != 0 {
                all_zeros = false;
            }
        }
        workbench[last_index] = 0;
        if all_zeros {
            break;
        }
        last_index -= 1;
    }

    let mut previous_last_value = 0;
    for last_number_for_turn in last_numbers_per_turn.iter().rev() {
        let new_last_value = previous_last_value + last_number_for_turn;
        previous_last_value = new_last_value;
    }

    previous_last_value
}

fn solve_history_part_2(history: &Vec<i64>) -> i64 {
    let mut workbench = history.clone();
    let mut last_index = workbench.len() - 1;

    let mut last_numbers_per_turn = Vec::new();

    loop {
        let mut all_zeros = true;
        // first element instead of last for part 2
        last_numbers_per_turn.push(workbench[0]);

        for i in 0..last_index {
            // difference is inverted for part 2
            let diff = workbench[i] - workbench[i + 1];
            workbench[i] = diff;
            if diff != 0 {
                all_zeros = false;
            }
        }
        workbench[last_index] = 0;
        if all_zeros {
            break;
        }
        last_index -= 1;
    }

    let mut previous_last_value = 0;
    for last_number_for_turn in last_numbers_per_turn.iter().rev() {
        let new_last_value = previous_last_value + last_number_for_turn;
        previous_last_value = new_last_value;
    }

    previous_last_value
}

pub fn day_9_part_1(data: &str) -> i64 {
    let (_, histories) = parse_input_data(data).expect("Failed to parse input data");
    histories
        .iter()
        .map(|history| solve_history_part_1(history))
        .sum()
}

/* easiest part 2 so far */
pub fn day_9_part_2(data: &str) -> i64 {
    let (_, histories) = parse_input_data(data).expect("Failed to parse input data");
    histories
        .iter()
        .map(|history| solve_history_part_2(history))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_day_9_part_1() {
        assert_eq!(day_9_part_1("0 3 6 9 12 15"), 18);
        assert_eq!(day_9_part_1("1 3 6 10 15 21"), 28);
        assert_eq!(day_9_part_1("10 13 16 21 30 45"), 68);
        assert_eq!(day_9_part_1(EXAMPLE), 114);
    }

    #[test]
    fn test_day_9_part_2() {
        assert_eq!(day_9_part_2("0 3 6 9 12 15"), -3);
        assert_eq!(day_9_part_2("1 3 6 10 15 21"), 0);
        assert_eq!(day_9_part_2("10 13 16 21 30 45"), 5);
        assert_eq!(day_9_part_2(EXAMPLE), 2);
    }
}
