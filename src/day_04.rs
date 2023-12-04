// First time using nom, let's see.
use nom::{
    self,
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    combinator::map,
    combinator::opt,
    multi::{many0, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

fn parse_card_title(data: &str) -> IResult<&str, u64> {
    map(
        tuple((tag("Card"), space1, nom::character::complete::u64)),
        |(_, _, number)| number,
    )(data)
}

fn parse_card(data: &str) -> IResult<&str, (u64, Vec<u8>, Vec<u8>)> {
    map(
        tuple((
            parse_card_title,
            tag(":"),
            space1,
            separated_list1(space1, nom::character::complete::u8),
            space1,
            tag("|"),
            space1,
            separated_list1(space1, nom::character::complete::u8),
        )),
        |(card_number, _, _, winning_numbers, _, _, _, card_numbers)| {
            (card_number, winning_numbers, card_numbers)
        },
    )(data)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<(u64, Vec<u8>, Vec<u8>)>> {
    many0(terminated(parse_card, opt(line_ending)))(data)
}

fn compute_nb_of_intersection(winning_numbers: &Vec<u8>, card_numbers: &Vec<u8>) -> i64 {
    // slow algorithm because the inputs are small and not worth
    // using fanciers algorithms with a bigger overhead like sets.
    let mut nb_of_intersection = 0;
    for winning_number in winning_numbers {
        if card_numbers.contains(winning_number) {
            nb_of_intersection += 1;
        }
    }
    return nb_of_intersection;
}

pub fn day_4_part_1(data: &str) -> i64 {
    let (_, cards) = parse_input_data(data).expect("Unable to parse input data");

    cards
        .iter()
        .map(|(_, winning_numbers, card_numbers)| {
            let nb = compute_nb_of_intersection(winning_numbers, card_numbers);
            if nb > 0 {
                return 2_i64.pow(nb as u32 - 1);
            }
            return 0;
        })
        .sum()
}

pub fn day_4_part_2(data: &str) -> i64 {
    let (_, cards) = parse_input_data(data).expect("Unable to parse input data");

    // create an array of numbers of cards filled with 1
    let mut collection = vec![1_i64; cards.len()];

    cards
        .iter()
        .enumerate()
        .for_each(|(card_number, (_, winning_numbers, card_numbers))| {
            let nb = compute_nb_of_intersection(winning_numbers, card_numbers);
            for i in card_number + 1..card_number + nb as usize + 1 {
                collection[i] += collection[card_number];
            }
        });

    return collection.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_day_4_part_1() {
        assert_eq!(day_4_part_1(EXAMPLE), 13);
    }

    #[test]
    fn test_day_4_part_2() {
        assert_eq!(day_4_part_2(EXAMPLE), 30);
    }
}
