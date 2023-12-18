use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, newline, space1, u32},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn parse(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, (_, _, times, _)) = tuple((
        tag("Time:"),
        multispace0,
        separated_list1(space1, u32),
        newline,
    ))(input)?;
    let (input, (_, _, distances, _)) = tuple((
        tag("Distance:"),
        multispace0,
        separated_list1(space1, u32),
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

fn calc_distance(time_pressed: u32, time_total: u32) -> u32 {
    let time_left = time_total - time_pressed;

    time_left * time_pressed
}

fn part1(input: &str) -> u32 {
    let (_, races) = parse(input).unwrap();

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
                .sum::<u32>()
        })
        .product::<u32>()
}

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    println!("Result part 1: {result}");
}

#[test]
fn test_part1() {
    let input = "Time:      7  15   30
Distance:  9  40  200
";
    assert_eq!(part1(input), 288);
}
