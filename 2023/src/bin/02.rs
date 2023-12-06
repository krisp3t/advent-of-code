advent_of_code::solution!(2);

enum CubeColor {
    Red,
    Green,
    Blue
}
struct Cubes {
    color: CubeColor,
    number: u8
}
struct Game {
    id: u32,
    pulls: Vec<Vec<Cubes>>
}
struct Games {
    games: Vec<Game>
}

pub fn parse_games(input: &str) -> Option<Games> {
    let mut games = Games { games: Vec::new() };
    for line in input.lines() {
        line.starts_with("Game").expect("Invalid input");
        let id = line.split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let mut pulls = Vec::new();
        for pull in line.split(";") {

        }
    }
    None
}
pub fn part_one(input: &str) -> Option<u32> {
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
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

