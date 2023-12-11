use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq)]
enum Point {
    Empty,
    Part(u32),
    Symbol(char),
}

#[derive(Debug)]
struct EnginePart {
    number: u32,
    symbol_present: bool,
}

fn part1(input: &str) -> u32 {
    let mut game: BTreeMap<(usize, usize), Point> = BTreeMap::new();
    let mut engine_parts: Vec<EnginePart> = Vec::new();

    let game_max_x = input.lines().count();
    let game_max_y = input.lines().next().unwrap().chars().count();

    let _: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let _: Vec<_> = line
                .chars()
                .enumerate()
                .map(|(x, ch)| {
                    let point = match ch {
                        '.' => Point::Empty,
                        part if ch.is_ascii_digit() => {
                            Point::Part(part.to_digit(10).expect("Not a number"))
                        }
                        _ => Point::Symbol(ch),
                    };

                    game.insert((y, x), point);
                })
                .collect();
        })
        .collect();

    let mut part_number = 0;
    let mut part_got_symbol = false;

    let _: Vec<_> = game
        .iter()
        .map(|((y, x), point)| {
            if *x == 0 && *y != 0 {
                /* start new line */
                if part_number != 0 {
                    engine_parts.push(EnginePart {
                        number: part_number,
                        symbol_present: part_got_symbol,
                    });
                }
                part_number = 0;
                part_got_symbol = false;
            }

            if let Point::Part(num) = *point {
                part_number = part_number * 10 + num;

                /* find if it touches a symbol */
                let neighbours: Vec<(i32, i32)> = vec![
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (-1, -1),
                    (1, -1),
                    (-1, 1),
                ];

                for (nx, ny) in neighbours.iter() {
                    let nx = nx + *x as i32;
                    let ny = ny + *y as i32;

                    if nx < 0 || ny < 0 {
                        continue;
                    }
                    if nx > (game_max_x - 1) as i32 || ny > (game_max_y - 1) as i32 {
                        continue;
                    }
                    let n = game.get(&(ny as usize, nx as usize)).unwrap();
                    if let Point::Symbol(_sym) = n {
                        part_got_symbol = true;
                    }
                }
            } else {
                /* start a new part */
                if part_number != 0 {
                    engine_parts.push(EnginePart {
                        number: part_number,
                        symbol_present: part_got_symbol,
                    });
                }
                part_number = 0;
                part_got_symbol = false;
            }
        })
        .collect();

    let total = engine_parts
        .iter()
        .filter(|part| part.symbol_present)
        .map(|part| part.number)
        .sum();

    total
}

fn part2(_input: &str) -> u32 {
    todo!();
}

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Result part 1: {result}");
}

#[test]
fn test_part1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(part1(&input), 4361);
}
