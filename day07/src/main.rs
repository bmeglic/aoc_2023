use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    High = 1,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Hand {
    cards: Vec<char>,
    typ: HandType,
    bid: u32,
}

fn cards_to_type_part1(cards: Vec<char>) -> HandType {
    let counts = cards.into_iter().counts();
    let counts = counts.values().sorted_by(|a, b| Ord::cmp(b, a));

    match counts.clone().next().unwrap() {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            match counts
                .into_iter()
                .nth(1)
                .expect("Failed to get 2nd element")
            {
                2 => HandType::FullHouse,
                1 => HandType::ThreeOfAKind,
                _ => panic!("wrong combination"),
            }
        }
        2 => {
            match counts
                .into_iter()
                .nth(1)
                .expect("Failed to get 2nd element")
            {
                2 => HandType::TwoPair,
                1 => HandType::OnePair,
                _ => panic!("wrong combination"),
            }
        }
        1 => HandType::High,
        _ => panic!("Not handled"),
    }
}

fn cards_to_type_part2(cards: Vec<char>) -> HandType {
    let mut counts = cards.into_iter().counts();
    counts.remove(&'J');
    let counts = counts.values().sorted_by(|a, b| Ord::cmp(b, a));

    match counts.clone().next() {
        Some(5) => HandType::FiveOfAKind,
        Some(4) => HandType::FourOfAKind,
        Some(3) => match counts.into_iter().nth(1) {
            Some(2) => HandType::FullHouse,
            Some(1) => HandType::ThreeOfAKind,
            None => HandType::ThreeOfAKind,
            _ => panic!("wrong combination"),
        },
        Some(2) => match counts.into_iter().nth(1) {
            Some(2) => HandType::TwoPair,
            Some(1) => HandType::OnePair,
            None => HandType::OnePair,
            _ => panic!("wrong combination"),
        },
        Some(1) => HandType::High,
        None => HandType::High,
        _ => panic!("Not handled"),
    }
}

fn card_to_value_part1(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        val => val.to_digit(10).expect("Unable to parse number"),
        //_ => panic!("Not able to convert card to value"),
    }
}

fn card_to_value_part2(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        val => val.to_digit(10).expect("Unable to parse number"),
        //_ => panic!("Not able to convert card to value"),
    }
}

fn parse_part1(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').expect("Unable to parse line");

            let cards = cards.chars().collect_vec();
            let typ = cards_to_type_part1(cards.clone());

            Hand {
                cards,
                typ,
                bid: bid.parse::<u32>().expect("Unable to parse number"),
            }
        })
        .collect()
}

fn parse_part2(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').expect("Unable to parse line");

            let cards = cards.chars().collect_vec();
            let typ = cards_to_type_part2(cards.clone());

            let counts = cards.iter().counts();
            let joker_count = counts.get(&'J');

            let typ = match (joker_count, typ) {
                (Some(1), HandType::High) => HandType::OnePair,
                (Some(1), HandType::OnePair) => HandType::ThreeOfAKind,
                (Some(1), HandType::TwoPair) => HandType::FullHouse,
                (Some(1), HandType::ThreeOfAKind) => HandType::FourOfAKind,
                (Some(1), HandType::FourOfAKind) => HandType::FiveOfAKind,

                (Some(2), HandType::High) => HandType::ThreeOfAKind,
                (Some(2), HandType::OnePair) => HandType::FourOfAKind,
                (Some(2), HandType::TwoPair) => HandType::FourOfAKind,
                (Some(2), HandType::ThreeOfAKind) => HandType::FiveOfAKind,
                (Some(2), HandType::FullHouse) => HandType::FiveOfAKind,

                (Some(3), HandType::High) => HandType::FourOfAKind,
                (Some(3), HandType::OnePair) => HandType::FiveOfAKind,
                (Some(3), HandType::FullHouse) => HandType::FiveOfAKind,

                (Some(4), HandType::High) => HandType::FiveOfAKind,
                (Some(4), HandType::FourOfAKind) => HandType::FiveOfAKind,

                (Some(5), HandType::High) => HandType::FiveOfAKind,
                (Some(5), HandType::FiveOfAKind) => HandType::FiveOfAKind,
                (None, typ) => typ,
                (cnt, typ) => {
                    dbg!(&cnt, &typ);
                    panic!("not implemented yet")
                }
            };

            Hand {
                cards,
                typ,
                bid: bid.parse::<u32>().expect("Unable to parse number"),
            }
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let hands = parse_part1(input);

    let hands = hands.into_iter().sorted_by(|a, b| {
        if a.typ > b.typ {
            Ordering::Greater
        } else if a.typ == b.typ {
            a.cards
                .iter()
                .zip(b.cards.iter())
                .filter_map(|(a, b)| {
                    let ord = card_to_value_part1(*a).cmp(&card_to_value_part1(*b));
                    if ord != Ordering::Equal {
                        Some(ord)
                    } else {
                        None
                    }
                })
                .next()
                .unwrap()
        } else {
            Ordering::Less
        }
    });

    hands
        .fold((1, 0), |mut acc, hand| {
            acc.1 += acc.0 * hand.bid;
            acc.0 += 1;

            acc
        })
        .1
}

fn part2(input: &str) -> u32 {
    let hands = parse_part2(input);

    let hands = hands.into_iter().sorted_by(|a, b| {
        if a.typ > b.typ {
            Ordering::Greater
        } else if a.typ == b.typ {
            a.cards
                .iter()
                .zip(b.cards.iter())
                .filter_map(|(a, b)| {
                    let ord = card_to_value_part2(*a).cmp(&card_to_value_part2(*b));
                    if ord != Ordering::Equal {
                        Some(ord)
                    } else {
                        None
                    }
                })
                .next()
                .unwrap()
        } else {
            Ordering::Less
        }
    });

    hands
        .fold((1, 0), |mut acc, hand| {
            acc.1 += acc.0 * hand.bid;
            acc.0 += 1;

            acc
        })
        .1

    /* 252127335 */
}

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Result part 1: {result}");
    let result = part2(input);
    println!("Result part 2: {result}");
}

#[test]
fn test_part1() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    assert_eq!(part1(input), 6440);
}

#[test]
fn test_part2() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    assert_eq!(part2(input), 5905);
}
