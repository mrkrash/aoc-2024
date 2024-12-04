use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re_mul = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let re_numbers = Regex::new(r"\d{1,3}").unwrap();
    Some(input.split('\n').fold(0, |acc: u32, line| {
        acc + re_mul.captures_iter(line).fold(0, |acc, mul| {
            acc + re_numbers
                .captures_iter(mul.get(0).unwrap().as_str())
                .fold(1, |acc, x| {
                    acc * x.get(0).unwrap().as_str().parse::<u32>().unwrap()
                })
        })
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_dont = Regex::new(r"(?s)don't\(\).*?(?:do\(\)|$)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Some(
        re_dont.captures_iter(input)
        .filter(|cap| !cap.get(1).is_none())
        .fold(0, |acc, cap| {
            acc + cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap()
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
