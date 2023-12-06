advent_of_code::solution!(2);

use std::collections::HashMap;
use nom::{
    IResult,
    sequence::preceded,
    bytes::complete::tag,
    character::complete::digit1,
};
use nom::multi::separated_list1;

#[derive(Debug, PartialEq, Eq, Hash)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

struct Cube {
    color: CubeColor,
    qty: usize,
}

impl Cube {
    // 3 blue
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, qty) = digit1(input)?;
        let (input, _) = tag(" ")(input)?;
        let mut color;
        match input {
            "red" => color = CubeColor::Red,
            "green" => color = CubeColor::Green,
            "blue" => color = CubeColor::Blue,
            _ => panic!("Unknown color"),
        }

        Ok((input, Self {
            color,
            qty: qty.parse().unwrap(),
        }))
    }
}

struct Round {
    cubes: Vec<Cube>,
}

impl Round {
    // 3 blue, 4 red
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, cubes) = separated_list1(tag(", "), Cube::parse)(input)?;
        Ok((input, Self {
            cubes,
        }))
    }
}

struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // -> id: 1
    // -> rounds: [[3 blue, 4 red], [1 red, 2 green, 6 blue], [2 green]]
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, id) = preceded(
            tag("Game "),
            digit1,
        )(input)?;
        let (input, rounds) = preceded(
            tag(": "),
            separated_list1(tag("; "), Round::parse))(input)?;
        ;

        Ok((input, Self {
            id: id.parse().unwrap(),
            rounds,
        }))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_cubes: HashMap<CubeColor, usize> = HashMap::from([
        (CubeColor::Red, 12),
        (CubeColor::Green, 13),
        (CubeColor::Blue, 14),
    ]);

    let possible_games: Vec<Game> = input
        .trim()
        .lines()
        .map(|g| Game::parse(g).expect("Parsing input failed").1)
        .collect();

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

