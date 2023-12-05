fn main() {
    //let input = include_str!("../input_example.txt");
    let input = include_str!("../input.txt");

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

    println!("Sum: {sum}");
}
