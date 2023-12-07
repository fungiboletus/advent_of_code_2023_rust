/*
    Part 1 sounds like a simple sort.
*/

use nom::{
    character::complete::{line_ending, one_of, space1, u64},
    combinator::{map, map_res},
    multi::{count, separated_list0},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    N2 = 2,
    N3 = 3,
    N4 = 4,
    N5 = 5,
    N6 = 6,
    N7 = 7,
    N8 = 8,
    N9 = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,

    JPART2 = 1,
}

impl Card {
    fn to_card_part_2(&self) -> Card {
        match self {
            Card::J => Card::JPART2,
            _ => *self,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl Hand {
    fn compute_hand_type_part_1(&mut self) {
        let mut counts_dictionary = [0_u8; 15];
        for card in self.cards.iter() {
            counts_dictionary[*card as usize] += 1;
        }
        let mut nb_pairs = 0;
        let mut has_a_three = false;
        for count in counts_dictionary.iter() {
            match *count {
                0..=1 => {}
                2 => nb_pairs += 1,
                3 => has_a_three = true,
                4 => {
                    self.hand_type = HandType::FourOfAKind;
                    return;
                }
                5 => {
                    self.hand_type = HandType::FiveOfAKind;
                    return;
                }
                _ => {
                    unreachable!("Invalid count");
                }
            }
        }
        if has_a_three {
            if nb_pairs == 1 {
                self.hand_type = HandType::FullHouse;
            } else if nb_pairs == 0 {
                self.hand_type = HandType::ThreeOfAKind;
            } else {
                unreachable!("Invalid count");
            }
        } else if nb_pairs == 2 {
            self.hand_type = HandType::TwoPairs;
        } else if nb_pairs == 1 {
            self.hand_type = HandType::OnePair;
        } else {
            self.hand_type = HandType::HighCard;
        }
    }

    fn to_part_2(&mut self) {
        for card in self.cards.iter_mut() {
            *card = card.to_card_part_2();
        }
    }

    fn compute_hand_type_part_2(&mut self) {
        let mut nb_jokers = 0;
        let mut counts_dictionary = [0_u8; 15];
        for card in self.cards.iter() {
            if *card == Card::J {
                unreachable!("Invalid card: J should have been converted to JPART2");
            } else if *card == Card::JPART2 {
                nb_jokers += 1;
            } else {
                counts_dictionary[*card as usize] += 1;
            }
        }
        let mut nb_pairs = 0;
        let mut has_a_three = false;
        let mut has_four = false;
        for count in counts_dictionary.iter() {
            match *count {
                0..=1 => {}
                2 => nb_pairs += 1,
                3 => has_a_three = true,
                4 => has_four = true,
                5 => {
                    self.hand_type = HandType::FiveOfAKind;
                    return;
                }
                _ => {
                    unreachable!("Invalid count: {}", count);
                }
            }
        }

        if has_four {
            if nb_jokers == 1 {
                self.hand_type = HandType::FiveOfAKind;
                return;
            }
            if nb_jokers == 0 {
                self.hand_type = HandType::FourOfAKind;
                return;
            }
            unreachable!("Invalid count: has four with more than one joker");
        }

        if has_a_three {
            if nb_jokers == 2 {
                self.hand_type = HandType::FiveOfAKind;
                return;
            }
            if nb_jokers == 1 {
                self.hand_type = HandType::FourOfAKind;
                return;
            }
            if nb_jokers == 0 {
                if nb_pairs == 1 {
                    self.hand_type = HandType::FullHouse;
                    return;
                }
                if nb_pairs == 0 {
                    self.hand_type = HandType::ThreeOfAKind;
                    return;
                }
                unreachable!("Invalid count: has a three with more than one pair");
            } else {
                unreachable!("Invalid count: has a three with more than two jokers");
            }
        }
        if nb_pairs == 2 {
            if nb_jokers == 1 {
                self.hand_type = HandType::FullHouse;
                return;
            }
            if nb_jokers == 0 {
                self.hand_type = HandType::TwoPairs;
                return;
            }
            unreachable!("Invalid count: has two pairs with more than one joker")
        }
        if nb_pairs == 1 {
            if nb_jokers == 3 {
                self.hand_type = HandType::FiveOfAKind;
                return;
            }
            if nb_jokers == 2 {
                self.hand_type = HandType::FourOfAKind;
                return;
            }
            if nb_jokers == 1 {
                self.hand_type = HandType::ThreeOfAKind;
                return;
            }
            if nb_jokers == 0 {
                self.hand_type = HandType::OnePair;
                return;
            }
            unreachable!("Invalid count: has one pair with more than three jokers")
        }
        if nb_jokers == 5 {
            self.hand_type = HandType::FiveOfAKind;
            return;
        }
        if nb_jokers == 4 {
            self.hand_type = HandType::FiveOfAKind;
            return;
        }
        if nb_jokers == 3 {
            self.hand_type = HandType::FourOfAKind;
            return;
        }
        if nb_jokers == 2 {
            self.hand_type = HandType::ThreeOfAKind;
            return;
        }
        if nb_jokers == 1 {
            self.hand_type = HandType::OnePair;
            return;
        }

        self.hand_type = HandType::HighCard;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ord_type_cmp = self.hand_type.cmp(&other.hand_type);
        if ord_type_cmp != std::cmp::Ordering::Equal {
            return Some(ord_type_cmp);
        }
        for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            let ord_card_cmp = card.cmp(other_card);
            if ord_card_cmp != std::cmp::Ordering::Equal {
                return Some(ord_card_cmp);
            }
        }
        return Some(std::cmp::Ordering::Equal);
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_hand(data: &str) -> IResult<&str, Hand> {
    map_res(count(one_of("AKQJT98765432"), 5), |cards_chars| {
        let mut cards = [Card::N2; 5];
        for (index, card_char) in cards_chars.iter().enumerate() {
            cards[index] = match card_char {
                '2' => Card::N2,
                '3' => Card::N3,
                '4' => Card::N4,
                '5' => Card::N5,
                '6' => Card::N6,
                '7' => Card::N7,
                '8' => Card::N8,
                '9' => Card::N9,
                'T' => Card::T,
                'J' => Card::J,
                'Q' => Card::Q,
                'K' => Card::K,
                'A' => Card::A,
                _ => {
                    return Err(nom::Err::Failure(nom::error::Error::new(
                        data,
                        nom::error::ErrorKind::OneOf,
                    )))
                }
            }
        }

        Ok(Hand {
            cards,
            hand_type: HandType::HighCard,
        })
    })(data)
}

fn parse_hand_and_bid(data: &str) -> IResult<&str, (Hand, u64)> {
    map(tuple((parse_hand, space1, u64)), |(hand, _, bid)| {
        (hand, bid)
    })(data)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<(Hand, u64)>> {
    separated_list0(line_ending, parse_hand_and_bid)(data)
}

fn compute_total_winnings(mut hands: Vec<(Hand, u64)>) -> i64 {
    hands.sort_by(|(hand, _), (other_hand, _)| hand.cmp(other_hand));

    hands
        .iter()
        .enumerate()
        .map(|(index, (_, bid))| {
            let rank = index + 1;
            return rank as i64 * *bid as i64;
        })
        .sum()
}

pub fn day_7_part_1(data: &str) -> i64 {
    let data = parse_input_data(data).expect("Unable to parse input data");

    let mut hands = data.1.clone();
    for (hand, _) in hands.iter_mut() {
        hand.compute_hand_type_part_1();
    }

    compute_total_winnings(hands)
}

pub fn day_7_part_2(data: &str) -> i64 {
    let data = parse_input_data(data).expect("Unable to parse input data");

    let mut hands = data.1.clone();
    for (hand, _) in hands.iter_mut() {
        hand.to_part_2();
        hand.compute_hand_type_part_2();
    }
    compute_total_winnings(hands)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_hand_type() {
        assert!(HandType::HighCard < HandType::OnePair);
        assert!(HandType::OnePair < HandType::TwoPairs);
        assert!(HandType::OnePair == HandType::OnePair);
    }

    #[test]
    fn test_day_7_part_1() {
        assert_eq!(day_7_part_1(EXAMPLE), 6440);
    }

    #[test]
    fn test_day_7_part_2() {
        assert_eq!(day_7_part_2(EXAMPLE), 5905);
    }

    #[test]
    fn test_part_2_stuff() {
        let mut hand = Hand {
            cards: [Card::Q, Card::J, Card::J, Card::Q, Card::N2],
            hand_type: HandType::HighCard,
        };
        hand.to_part_2();
        hand.compute_hand_type_part_2();
        assert_eq!(hand.hand_type, HandType::FourOfAKind);

        let mut hand_weak = Hand {
            cards: [Card::J, Card::K, Card::K, Card::K, Card::N2],
            hand_type: HandType::HighCard,
        };
        hand_weak.to_part_2();
        hand_weak.compute_hand_type_part_2();

        let mut hand_strong = Hand {
            cards: [Card::Q, Card::Q, Card::Q, Card::Q, Card::N2],
            hand_type: HandType::HighCard,
        };
        hand_strong.to_part_2();
        hand_strong.compute_hand_type_part_2();
        assert!(hand_weak < hand_strong);
    }

    #[test]
    fn extra_test_case() {
        const EXTRA: &str = "AAAAA 2
22222 3
AAAAK 5
22223 7
AAAKK 11
22233 13
AAAKQ 17
22234 19
AAKKQ 23
22334 29
AAKQJ 31
22345 37
AKQJT 41
23456 43";
        assert_eq!(day_7_part_1(EXTRA), 1343);
        assert_eq!(day_7_part_2(EXTRA), 1369);
    }

    #[test]
    fn test_broken_case() {
        const BROKEN: &str = "22334 2\nAAQKJ 1";
        assert_eq!(day_7_part_2(BROKEN), 4);
    }

    #[test]
    fn test_other_broken_case() {
        const BROKEN: &str = "23456 2\nAKQJT 1";
        assert_eq!(day_7_part_2(BROKEN), 4);
    }

    #[test]
    fn test_jjjjj() {
        const BROKEN: &str = "23456 2\nJJJJJ 1";
        assert_eq!(day_7_part_2(BROKEN), 4);
    }

    #[test]
    fn test_qqjja() {
        const BROKEN: &str = "QQQQ3 2\nQQJJA 1";
        assert_eq!(day_7_part_2(BROKEN), 5);
    }

    #[test]
    fn test_jjjja() {
        const BROKEN: &str = "QQQQ3 2\nJJJJA 1";
        assert_eq!(day_7_part_2(BROKEN), 4);
    }
}
