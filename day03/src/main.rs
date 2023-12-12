use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Point {
    Empty,
    Part(u32),
    Symbol(char),
}

#[derive(Debug, Default, Clone)]
struct EnginePart {
    number: u32,
    symbols_adjacent: HashMap<(usize, usize), char>,
}

fn parse(input: &str) -> Vec<EnginePart> {
    let mut engine_parts: Vec<EnginePart> = Vec::new();
    let mut game: BTreeMap<(usize, usize), Point> = BTreeMap::new();
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
    let mut symbols_adjacent: HashMap<(usize, usize), char> = HashMap::new();

    let _: Vec<_> = game
        .iter()
        .map(|((y, x), point)| {
            if *x == 0 && *y != 0 {
                /* start new line */
                if part_number != 0 {
                    engine_parts.push(EnginePart {
                        number: part_number,
                        symbols_adjacent: symbols_adjacent.clone(),
                    });
                }
                symbols_adjacent.clear();
                part_number = 0;
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
                    if let Point::Symbol(sym) = n {
                        symbols_adjacent.insert((nx as usize, ny as usize), *sym);
                    }
                }
            } else {
                /* start a new part */
                if part_number != 0 {
                    engine_parts.push(EnginePart {
                        number: part_number,
                        symbols_adjacent: symbols_adjacent.clone(),
                    });
                }
                symbols_adjacent.clear();
                part_number = 0;
            }
        })
        .collect();

    engine_parts
}

fn part1(input: &str) -> u32 {
    let engine_parts = parse(input);

    let total = engine_parts
        .iter()
        .filter(|part| !part.symbols_adjacent.is_empty())
        .map(|part| part.number)
        .sum();

    total
}

fn part2(input: &str) -> u32 {
    let engine_parts = parse(input);

    let engine_parts_with_star_symbol: Vec<_> = engine_parts
        .iter()
        .filter(|part| part.symbols_adjacent.values().any(|sym| *sym == '*'))
        .collect();

    engine_parts_with_star_symbol
        .iter()
        .enumerate()
        .map(|(idx, part1)| {
            let sym1 = part1.symbols_adjacent.iter().find(|sym| *sym.1 == '*');

            if let Some(sym1) = sym1 {
                if let Some(part2) = engine_parts_with_star_symbol
                    .iter()
                    .skip(idx + 1)
                    .find(|part| part.symbols_adjacent.iter().any(|sym| sym == sym1))
                {
                    return part1.number * part2.number;
                };
            };

            0
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

#[test]
fn test_part2() {
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
    assert_eq!(part2(&input), 467835);
}
