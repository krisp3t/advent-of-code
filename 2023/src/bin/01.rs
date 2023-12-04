advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
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
    let sum: u32 = input
        .trim()
        .lines()
        .map(|line|
            line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        )
        .filter_map(|line| {
            let first_digit = line.chars().find(|c| c.is_digit(10))?;
            let last_digit = line.chars().rev().find(|c| c.is_digit(10))?;
            format!("{}{}", first_digit, last_digit).parse::<u32>().ok()
        })
        .sum();
    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
