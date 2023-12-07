fn part1(input: &str) -> u32 {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let first = line.find(|c: char| c.is_ascii_digit());
            let last = line.rfind(|c: char| c.is_ascii_digit());

            if first.is_none() || last.is_none() {
                panic!("We need to have both start and end digit!")
            }

            let first = line
                .chars()
                .nth(first.unwrap())
                .map(|c| c.to_digit(10))
                .unwrap()
                .unwrap();
            let last = line
                .chars()
                .nth(last.unwrap())
                .map(|c| c.to_digit(10))
                .unwrap()
                .unwrap();

            (first * 10) + last
        })
        .sum();

    sum
}

fn part2(input: &str) -> u32 {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let sum: u32 = input
        .lines()
        .map(|line| {
            let mut digits: Vec<u32> = Vec::new();

            for idx in 0..line.len() {
                let new_line = &line[idx..];

                if let Some(digit) = new_line.chars().next().unwrap().to_digit(10) {
                    digits.push(digit);
                } else {
                    for word in 0..words.len() {
                        if new_line.starts_with(words[word]) {
                            digits.push(word as u32);
                        }
                    }
                }
            }

            let first = digits.first().unwrap();
            let last = digits.last().unwrap();

            (first * 10) + last
        })
        .sum();

    sum
}

fn main() {
    //let input = include_str!("../input_example2.txt");
    let input = include_str!("../input.txt");

    //let sum = part1(input);
    let sum = part2(input);

    println!("Sum: {sum}");
}
