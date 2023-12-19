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

fn cards_to_type(cards: Vec<char>) -> HandType {
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

fn card_to_value(card: char) -> u32 {
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

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').expect("Unable to parse line");

            let cards = cards.chars().collect_vec();
            let typ = cards_to_type(cards.clone());

            Hand {
                cards,
                typ,
                bid: bid.parse::<u32>().expect("Unable to parse number"),
            }
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let hands = parse(input);

    let hands = hands.into_iter().sorted_by(|a, b| {
        if a.typ > b.typ {
            Ordering::Greater
        } else if a.typ == b.typ {
            a.cards
                .iter()
                .zip(b.cards.iter())
                .filter_map(|(a, b)| {
                    let ord = card_to_value(*a).cmp(&card_to_value(*b));
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

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Result part 1: {result}");
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
