use std::collections::HashSet;

#[derive(Debug, Default)]
struct Card {
    _id: u32,
    num_win: HashSet<u32>,
    num_own: HashSet<u32>,
}

fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (id, rest) = line.split_once(':').expect("Cannot parse line");
            let id = id
                .split_ascii_whitespace()
                .next_back()
                .expect("Cannot parse line")
                .parse::<u32>()
                .expect("Cannot parse id");

            let (num_win, num_own) = rest.split_once('|').expect("Cannot parse line");

            let num_win: HashSet<_> = num_win
                .split_ascii_whitespace()
                .map(|str| str.parse::<u32>().expect("Cannot parse number"))
                .collect();
            let num_own: HashSet<_> = num_own
                .split_ascii_whitespace()
                .map(|str| str.parse::<u32>().expect("Cannot parse number"))
                .collect();

            Card {
                _id: id,
                num_win,
                num_own,
            }
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let games = parse(input);

    games
        .iter()
        .map(|card| {
            let count = card.num_win.intersection(&card.num_own).count();
            if count > 0 {
                2u32.pow(count as u32 - 1)
            } else {
                0
            }
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let games = parse(input);

    let winning_count: Vec<u32> = games
        .iter()
        .map(|card| card.num_win.intersection(&card.num_own).count() as u32)
        .collect();

    winning_count
        .iter()
        .enumerate()
        .scan(vec![1; winning_count.len()], |counts, (idx, count)| {
            let num_cards = counts[idx];
            for i in idx + 1..=idx + *count as usize {
                counts[i] += num_cards;
            }
            Some(num_cards)
        })
        .sum()
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
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(part1(input), 13);
}

#[test]
fn test_part2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(part2(input), 30);
}
