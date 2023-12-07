use std::iter::Sum;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Debug)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl Sum for GameSet {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = GameSet>,
    {
        let mut total = GameSet {
            red: 0,
            green: 0,
            blue: 0,
        };

        for set in iter {
            total.red += set.red;
            total.green += set.green;
            total.blue += set.blue;
        }

        total
    }
}

impl AddAssign for GameSet {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        };
    }
}

impl FromStr for GameSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<GameSet, Self::Err> {
        let set = s
            .split(", ")
            .map(|value| {
                let (num, color) = value.split_once(' ').unwrap();
                let num = num.parse::<u32>().unwrap();

                if color == "red" {
                    GameSet {
                        red: num,
                        green: 0,
                        blue: 0,
                    }
                } else if color == "green" {
                    GameSet {
                        red: 0,
                        green: num,
                        blue: 0,
                    }
                } else if color == "blue" {
                    GameSet {
                        red: 0,
                        green: 0,
                        blue: num,
                    }
                } else {
                    panic!("Error parsing colors");
                }
            })
            .sum::<GameSet>();

        Ok(set)
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Game, Self::Err> {
        let (part_game_id, part_sets) = s.split_once(": ").unwrap();

        let id = part_game_id
            .get(
                part_game_id
                    .find(char::is_numeric)
                    .expect("Could not find game id")..,
            )
            .expect("Could not get game id")
            .parse::<u32>()?;

        let sets: Result<Vec<GameSet>, anyhow::Error> = part_sets
            .split("; ")
            .map(|s| GameSet::from_str(s))
            .collect();
        let sets = sets?;

        Ok(Game { id, sets })
    }
}

fn part1(input: &str) -> u32 {
    let max_possible_game = GameSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games: Result<Vec<Game>, anyhow::Error> =
        input.lines().map(|l| Game::from_str(l)).collect();

    let mut total = 0;

    for game in games.unwrap() {
        let mut possible = true;

        for game_set in game.sets {
            if game_set.red > max_possible_game.red
                || game_set.green > max_possible_game.green
                || game_set.blue > max_possible_game.blue
            {
                possible = false;
                break;
            }
        }

        if possible {
            total += game.id;
        }
    }

    total
}

fn _part2(_input: &str) -> u32 {
    todo!();
}

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(input);
    //let result = part2(input);

    println!("Result: {result}");
}

#[test]
fn test_part1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part1(&input), 8);
}
