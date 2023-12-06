advent_of_code::solution!(2);

use std::collections::HashMap;
use nom::{
    IResult,
    sequence::preceded,
    bytes::complete::tag,
    character::complete::digit1,
};

#[derive(Debug, PartialEq, Eq, Hash)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

struct Cubes {
    color: CubeColor,
    qty: usize,
}

struct Game {
    id: usize,
    pulls: Vec<Vec<Cubes>>,
}

impl Game {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, id) = preceded(
            tag("Game "),
            digit1,
        )(input)?;

        Ok((input, Self {
            id: id.parse().unwrap(),
            pulls: Vec::new(),
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

