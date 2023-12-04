advent_of_code::solution!(1);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"[0-9]").unwrap();
    let sum: u32 = input
        .trim()
        .lines()
        .filter_map(|line| {
            let first_digit = line.chars().find(|c| c.is_digit(10))?;
            let last_digit = line.chars().rev().find(|c| c.is_digit(10))?;
            format!("{}{}", first_digit, last_digit).parse::<u32>().ok()
        })
        .sum();
    return Some(sum);
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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
