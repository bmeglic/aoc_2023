use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{multispace0, multispace1, u64},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};
use std::ops::Range;

#[derive(Debug)]
struct Mapping {
    src: Range<u64>,
    dst: Range<u64>,
}

impl Mapping {
    fn translate(&self, input: u64) -> Option<u64> {
        if self.src.contains(&input) {
            let diff = input - self.src.start;
            Some(self.dst.start + diff)
        } else {
            None
        }
    }
}

fn parse_map(input: &str) -> IResult<&str, Vec<Mapping>> {
    let (input, (_, _, _, values)) = tuple((
        take_until("map:"),
        tag("map:"),
        multispace1,
        separated_list1(multispace0, u64),
    ))(input)?;

    let mappings: Vec<_> = values
        .chunks(3)
        .map(|v| Mapping {
            src: v[1]..v[1] + v[2],
            dst: v[0]..v[0] + v[2],
        })
        .collect();

    Ok((input, (mappings)))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<Vec<Mapping>>)> {
    let (input, (_, seeds)) = tuple((tag("seeds: "), separated_list1(multispace0, u64)))(input)?;

    let (input, mappings) = many1(parse_map)(input)?;

    Ok((input, (seeds, mappings)))
}

fn part1(input: &str) -> u64 {
    let (_input, (seeds, mappings)) = parse_input(input).expect("Unable to parse input");

    seeds
        .iter()
        .map(|seed| {
            let result = mappings.iter().fold(*seed, |acc, mapping| {
                for map in mapping {
                    if let Some(result) = map.translate(acc) {
                        return result;
                    }
                }
                acc
            });
            result
        })
        .min()
        .expect("There was an error finding out minimum")
}

fn part2(input: &str) -> u64 {
    let (_input, (seeds, mappings)) = parse_input(input).expect("Unable to parse input");

    let seeds_ranges: Vec<_> = seeds
        .chunks(2)
        .map(|v| Range {
            start: v[0],
            end: v[0] + v[1],
        })
        .collect();

    seeds_ranges
        .iter()
        .map(|seed_mapping| {
            seed_mapping
                .clone()
                .into_iter()
                .map(|seed| {
                    let result = mappings.iter().fold(seed, |acc, mapping| {
                        for map in mapping {
                            if let Some(result) = map.translate(acc) {
                                return result;
                            }
                        }
                        acc
                    });
                    result
                })
                .min()
                .expect("There was an error finding out minimum")
        })
        .min()
        .expect("There was an error finding out minimum")
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
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
    assert_eq!(part1(input), 35);
}

#[test]
fn test_part2() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
    assert_eq!(part2(input), 46);
}
