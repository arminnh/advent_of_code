use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs;
use std::str::Lines;
use std::usize;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(()),
        }
    }
}

fn parse_hand(s: &str) -> Vec<Card> {
    s.chars().map(|c| Card::try_from(c).unwrap()).collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn from_hand(hand: &Vec<Card>) -> Self {
        let mut counts: HashMap<&Card, usize> = HashMap::new();
        hand.iter()
            .for_each(|item| *counts.entry(item).or_insert(0) += 1);

        let mut counts_ordered: Vec<&usize> = counts.values().collect();
        counts_ordered.sort_by(|a, b| b.cmp(a));

        if let Some(first) = counts_ordered.first() {
            match first {
                5 => Self::FiveOfAKind,
                4 => Self::FourOfAKind,
                _ => {
                    let second = counts_ordered.get(1).unwrap();
                    match (first, second) {
                        (3, 2) => Self::FullHouse,
                        (3, 1) => Self::ThreeOfAKind,
                        (2, 2) => Self::TwoPair,
                        (2, 1) => Self::OnePair,
                        _ => Self::HighCard,
                    }
                }
            }
        } else {
            panic!("Cannot determine hand type.")
        }
    }
}

fn part_1(lines: Lines) -> usize {
    let mut parsed: Vec<(Vec<Card>, HandType, usize)> = lines
        .map(
            |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
                [hand_str, bet_str] => {
                    let hand = parse_hand(hand_str);
                    let hand_type = HandType::from_hand(&hand);
                    let bet: usize = bet_str.parse().unwrap();

                    (hand, hand_type, bet)
                }
                _ => panic!("Unsupported input: {:?}", line),
            },
        )
        .collect();

    parsed.sort_by(
        |(left_hand, left_hand_type, _), (right_hand, right_hand_type, _)| {
            if left_hand_type == right_hand_type {
                right_hand.cmp(left_hand)
            } else {
                right_hand_type.cmp(left_hand_type)
            }
        },
    );

    parsed
        .iter()
        .enumerate()
        .fold(0, |result, (index, (_, _, bet))| result + (parsed.len() - index) * bet)
}

pub fn solve() -> SolutionPair {
    let contents = fs::read_to_string("inputs/day_7").expect("Could not open file.");

    (Solution::from(part_1(contents.lines())), Solution::from(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        assert_eq!(part_1(input.lines()), 6440);
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(
            parse_hand("AKQJT"),
            vec![Card::Ace, Card::King, Card::Queen, Card::Jack, Card::Ten]
        );
        assert_eq!(
            parse_hand("98765"),
            vec![Card::Nine, Card::Eight, Card::Seven, Card::Six, Card::Five]
        );
        assert_eq!(parse_hand("43"), vec![Card::Four, Card::Three]);
        assert_eq!(parse_hand("2"), vec![Card::Two]);
    }

    #[test]
    fn test_card_order() {
        let mut all_cards = parse_hand("AKQJT98765432");
        all_cards.sort();
        assert_eq!(all_cards, parse_hand("23456789TJQKA"));
    }

    #[test]
    fn test_from_hand() {
        assert_eq!(
            HandType::from_hand(&parse_hand("AKQJT")),
            HandType::HighCard
        );
        assert_eq!(
            HandType::from_hand(&parse_hand("23456")),
            HandType::HighCard
        );
        assert_eq!(HandType::from_hand(&parse_hand("32T3K")), HandType::OnePair);
        assert_eq!(HandType::from_hand(&parse_hand("KK28A")), HandType::OnePair);
        assert_eq!(HandType::from_hand(&parse_hand("KK677")), HandType::TwoPair);
        assert_eq!(HandType::from_hand(&parse_hand("KTJJT")), HandType::TwoPair);
        assert_eq!(
            HandType::from_hand(&parse_hand("T55J5")),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            HandType::from_hand(&parse_hand("QQQJA")),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            HandType::from_hand(&parse_hand("T55T5")),
            HandType::FullHouse
        );
        assert_eq!(
            HandType::from_hand(&parse_hand("QQQJJ")),
            HandType::FullHouse
        );
        assert_eq!(
            HandType::from_hand(&parse_hand("TTTTA")),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_hand(&parse_hand("2AAAA")),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_hand(&parse_hand("AAAAA")),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandType::from_hand(&parse_hand("99999")),
            HandType::FiveOfAKind
        );
    }
}
