use itertools::{self, Itertools};

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|val| val.parse::<i64>().expect("Unable to parse"))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let histories = parse(input);

    histories
        .iter()
        .map(|history| {
            let mut old_history = history.clone();
            let mut edge: Vec<i64> = Vec::new();

            loop {
                let new_history: Vec<_> = old_history
                    .iter()
                    .tuple_windows()
                    .map(|(left, right)| right - left)
                    .collect();

                edge.push(*new_history.last().unwrap());

                if new_history.iter().all(|history| history == &0) {
                    break;
                }

                old_history = new_history;
            }

            history.last().unwrap() + edge.iter().sum::<i64>()
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Result part 1: {result}");
}

#[test]
fn test_part1() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    assert_eq!(part1(input), 114);
}
