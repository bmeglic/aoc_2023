use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, newline},
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

fn parse_location(input: &str) -> IResult<&str, Location> {
    let (input, (_, loc, _, left, _, right, _)) = tuple((
        multispace0,
        alpha1,
        tag(" = ("),
        alpha1,
        tag(", "),
        alpha1,
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

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Result part 1: {result}");
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
