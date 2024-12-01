advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut column_1: Vec<u32> = Vec::new();
    let mut column_2: Vec<u32> = Vec::new();
    input
        .split('\n')
        .map(|s| s.split_whitespace())
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|mut v| {
            column_1.push(v.next().unwrap().parse().unwrap());
            column_2.push(v.next().unwrap().parse().unwrap());
        });
    column_1.sort();
    column_2.sort();
    (column_1, column_2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (column_1, column_2) = parse_input(input);
    Some(
        column_1
            .iter()
            .zip(column_2.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (column_1, column_2) = parse_input(input);
    Some(
        column_1
            .iter()
            .map(|x| {
                x * column_2
                    .iter()
                    .fold(0, |acc, y| if x == y { acc + 1 } else { acc })
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
