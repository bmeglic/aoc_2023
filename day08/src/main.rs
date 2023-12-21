use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0, newline},
    multi::many1,
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Left = 0,
    Right = 1,
}

#[derive(Debug)]
struct Location {
    loc: String,
    left: String,
    right: String,
}

#[derive(Debug)]
struct Ghost {
    loc: String,
    cycles: Option<u64>,
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn parse_location(input: &str) -> IResult<&str, Location> {
    let (input, (_, loc, _, left, _, right, _)) = tuple((
        multispace0,
        alphanumeric1,
        tag(" = ("),
        alphanumeric1,
        tag(", "),
        alphanumeric1,
        tag(")"),
    ))(input)?;

    Ok((
        input,
        Location {
            loc: loc.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        },
    ))
}

fn parse(input: &str) -> IResult<&str, (Vec<Direction>, HashMap<String, Location>)> {
    let (input, (dirs, _)) = tuple((alpha1, newline))(input)?;
    let directions: Vec<Direction> = dirs
        .chars()
        .map(|dir| match dir {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("unknown character"),
        })
        .collect();

    let (input, locations) = many1(parse_location)(input)?;
    let locations = locations
        .iter()
        .map(|loc| {
            (
                loc.loc.clone(),
                Location {
                    loc: loc.loc.clone(),
                    left: loc.left.clone(),
                    right: loc.right.clone(),
                },
            )
        })
        .collect();

    Ok((input, (directions, locations)))
}

fn part1(input: &str) -> u32 {
    let (_, (directions, locations)) = parse(input).unwrap();

    let mut cur_loc = locations
        .get(&"AAA".to_string())
        .expect("Could not get starting position");

    directions
        .iter()
        .cycle()
        .map_while(|dir| {
            let next_loc = match dir {
                Direction::Left => &cur_loc.left,
                Direction::Right => &cur_loc.right,
            };

            cur_loc = locations
                .get(next_loc)
                .expect("Could not get next location");
            if cur_loc.loc == "ZZZ" {
                None
            } else {
                Some(1)
            }
        })
        .sum::<u32>()
        + 1
}

fn part2(input: &str) -> u64 {
    let (_, (directions, locations)) = parse(input).unwrap();

/* copied from:
 * https://dev.to/nickymeuleman/advent-of-code-2023-day-8-255k
 */
    let mut cycle_count = 0;
    let mut ghosts: Vec<Ghost> = locations
        .keys()
        .filter(|loc| loc.ends_with('A'))
        .map(|loc| Ghost {
            loc: loc.clone(),
            cycles: None,
        })
        .collect();


    while ghosts.iter().any(|ghost| ghost.cycles.is_none()) {
        // Do a full cycle of instructions
        for dir in &directions {
            for Ghost { loc, cycles } in ghosts.iter_mut() {
                if cycles.is_some() {
                    // this loop already has a known cycle length, no need to simulate further
                    continue;
                }

                let tmp = locations.get(loc).expect("Could not find element");

                let next_loc = match dir {
                    Direction::Left => &tmp.left,
                    Direction::Right => &tmp.right,
                };
                *loc = locations
                    .get(next_loc)
                    .expect("Could not get next location").loc.clone();

            }
        }
        cycle_count += 1;

        // after a full cycle of instructions, save any found cycles (ghosts that arrived at a destination)
        for Ghost { loc, cycles: cycle } in ghosts.iter_mut() {
            if cycle.is_some() {
                // already has a known cycle, no need to update
                continue;
            }
            if loc.ends_with('Z') {
                *cycle = Some(cycle_count);
            }
        }
    }

    let min_shared_cycles = ghosts
        .into_iter()
        .filter_map(|ghost| ghost.cycles)
        .fold(1, |acc, item| lcm(acc, item));

    min_shared_cycles * directions.len() as u64

    /*
        basic dumb implementation

        let mut cur_locs: HashMap<String, Location> = locations
            .iter()
            .filter(|loc| loc.0.ends_with('A'))
            .map(|(key, val)| {
                (
                    key.clone(),
                    Location {
                        loc: val.loc.clone(),
                        left: val.left.clone(),
                        right: val.right.clone(),
                    },
                )
            })
            .collect();

        directions
            .iter()
            .cycle()
            .map_while(|dir| {
                cur_locs = cur_locs
                    .values()
                    .map(|val| {
                        let next_loc = match dir {
                            Direction::Left => &val.left,
                            Direction::Right => &val.right,
                        };
                        let next_loc = locations
                            .get(next_loc)
                            .expect("Could not get next location");

                        (
                            next_loc.loc.clone(),
                            Location {
                                loc: next_loc.loc.clone(),
                                left: next_loc.left.clone(),
                                right: next_loc.right.clone(),
                            },
                        )
                    })
                    .collect();

                cur_locs
                    .iter()
                    .any(|loc| !loc.0.ends_with('Z'))
                    .then_some(1)
            })
            .sum::<u32>()
            + 1
    */
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
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
    assert_eq!(part1(input), 6);
}

#[test]
fn test_part2() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
    assert_eq!(part2(input), 6);
}
