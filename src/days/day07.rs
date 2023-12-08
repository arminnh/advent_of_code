use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs;
use std::str::Lines;
use std::usize;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
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

fn parse_cards(s: &str) -> Vec<Card> {
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
    pub fn from_cards(hand: &Vec<Card>, promote_jack: bool) -> Self {
        let mut counts: HashMap<&Card, usize> = HashMap::new();
        hand.iter()
            .for_each(|item| *counts.entry(item).or_insert(0) += 1);

        if promote_jack && counts.len() > 1 {
            if let Some(jack_count) = counts.remove(&Card::Jack) {
                let (max_card, _) = counts.iter().max_by_key(|(_, count)| *count).unwrap();
                *counts.entry(max_card).or_insert(0) += jack_count;
            }
        }

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

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bet: usize,
}

fn parse_hands(lines: Lines, promote_jack: bool) -> Vec<Hand> {
    lines
        .map(
            |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
                [hand_str, bet_str] => {
                    let cards = parse_cards(hand_str);
                    let hand_type = HandType::from_cards(&cards, promote_jack);
                    let bet: usize = bet_str.parse().unwrap();

                    Hand {
                        cards,
                        hand_type,
                        bet,
                    }
                }
                _ => panic!("Unsupported input: {:?}", line),
            },
        )
        .collect()
}

fn sort_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(|left, right| {
        if left.hand_type == right.hand_type {
            right.cards.cmp(&left.cards)
        } else {
            right.hand_type.cmp(&left.hand_type)
        }
    });
}

fn calculate_result(hands: Vec<Hand>) -> usize {
    hands.iter().enumerate().fold(0, |result, (index, hand)| {
        let rank = hands.len() - index;

        result + rank * hand.bet
    })
}

fn part_1(lines: Lines) -> usize {
    let mut hands: Vec<Hand> = parse_hands(lines, false);
    sort_hands(&mut hands);
    calculate_result(hands)
}

fn part_2(lines: Lines) -> usize {
    let mut hands: Vec<Hand> = parse_hands(lines, true);
    sort_hands(&mut hands);
    calculate_result(hands)
}

pub fn solve() -> SolutionPair {
    let contents = fs::read_to_string("inputs/day_7").expect("Could not open file.");

    (
        Solution::from(part_1(contents.lines())),
        Solution::from(part_2(contents.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 6440);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 5905);
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(
            parse_cards("AKQJT"),
            vec![Card::Ace, Card::King, Card::Queen, Card::Jack, Card::Ten]
        );
        assert_eq!(
            parse_cards("98765"),
            vec![Card::Nine, Card::Eight, Card::Seven, Card::Six, Card::Five]
        );
        assert_eq!(parse_cards("43"), vec![Card::Four, Card::Three]);
        assert_eq!(parse_cards("2"), vec![Card::Two]);
    }

    #[test]
    fn test_card_order() {
        let mut all_cards = parse_cards("AKQJT98765432");
        all_cards.sort();
        assert_eq!(all_cards, parse_cards("J23456789TQKA"));
    }

    #[test]
    fn test_hand_type_parsing() {
        assert_eq!(
            HandType::from_cards(&parse_cards("AKQJT"), false),
            HandType::HighCard
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("23456"), false),
            HandType::HighCard
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("32T3K"), false),
            HandType::OnePair
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("KK28A"), false),
            HandType::OnePair
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("KK677"), false),
            HandType::TwoPair
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("KTJJT"), false),
            HandType::TwoPair
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("T55J5"), false),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("QQQJA"), false),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("T55T5"), false),
            HandType::FullHouse
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("QQQJJ"), false),
            HandType::FullHouse
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("TTTTA"), false),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("2AAAA"), false),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("AAAAA"), false),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("99999"), false),
            HandType::FiveOfAKind
        );
    }

    #[test]
    fn test_hand_type_parsing_promoted_jack() {
        assert_eq!(
            HandType::from_cards(&parse_cards("AKQJT"), true),
            HandType::OnePair
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("23456"), true),
            HandType::HighCard
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("32T3K"), true),
            HandType::OnePair
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("KTJJT"), true),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("T55J5"), true),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("QQQJA"), true),
            HandType::FourOfAKind
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("T55T5"), true),
            HandType::FullHouse
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("QQQJJ"), true),
            HandType::FiveOfAKind
        );
        assert_eq!(
            HandType::from_cards(&parse_cards("TTTTJ"), true),
            HandType::FiveOfAKind
        );
    }
}
