use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, newline, space1, u64},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_part1(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, (_, _, times, _)) = tuple((
        tag("Time:"),
        multispace0,
        separated_list1(space1, u64),
        newline,
    ))(input)?;
    let (input, (_, _, distances, _)) = tuple((
        tag("Distance:"),
        multispace0,
        separated_list1(space1, u64),
        newline,
    ))(input)?;

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|v| Race {
            time: *v.0,
            distance: *v.1,
        })
        .collect();

    Ok((input, races))
}

fn parse_part2(input: &str) -> IResult<&str, Race> {
    let (input, (_, _, times, _)) = tuple((
        tag("Time:"),
        multispace0,
        separated_list1(space1, digit1),
        newline,
    ))(input)?;
    let (input, (_, _, distances, _)) = tuple((
        tag("Distance:"),
        multispace0,
        separated_list1(space1, digit1),
        newline,
    ))(input)?;

    let time = times
        .into_iter()
        .collect::<String>()
        .parse::<u64>()
        .expect("Unable to convert number");

    let distance = distances
        .into_iter()
        .collect::<String>()
        .parse::<u64>()
        .expect("Unable to convert number");

    Ok((input, Race {
        time,
        distance,
    }))
}

fn calc_distance(time_pressed: u64, time_total: u64) -> u64 {
    let time_left = time_total - time_pressed;

    time_left * time_pressed
}

fn part1(input: &str) -> u64 {
    let (_, races) = parse_part1(input).unwrap();

    races
        .iter()
        .map(|race| {
            (0..race.time)
                .map(|v| {
                    if calc_distance(v, race.time) > race.distance {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .product::<u64>()
}

fn part2(input: &str) -> u64 {
    let (_, race) = parse_part2(input).unwrap();

    (0..race.time)
        .map(|v| {
            if calc_distance(v, race.time) > race.distance {
                1
            } else {
                0
            }
        })
        .sum::<u64>()
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
    let input = "Time:      7  15   30
Distance:  9  40  200
";
    assert_eq!(part1(input), 288);
}

#[test]
fn test_part2() {
    let input = "Time:      7  15   30
Distance:  9  40  200
";
    assert_eq!(part2(input), 71503);
}
