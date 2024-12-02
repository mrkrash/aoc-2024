advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .map(|line| {
                let mut report = line
                    .split(' ')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                if is_safe(&report) {
                    return true;
                }
                report.reverse();
                is_safe(&report)
            })
            .filter(|safe| safe == &true)
            .count() as u32,
    )
}

fn is_safe(report: &Vec<i32>) -> bool {
    if report.is_sorted()
        && report.iter().fold(0, |acc: i32, x| {
            if acc == 0 {
                return *x;
            } else if x - acc > 3 || acc - x > 3 || x - acc == 0 {
                return -99;
            }
            *x
        }) > -99
    {
        return true;
    }
    false
}
fn is_safe_tolerated(report: &Vec<i32>) -> bool {
    for i in 0..report.iter().count() {
        let mut _report = report.clone();
        _report.remove(i);
        if is_safe(&_report) {
            return true;
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .map(|line| {
                let mut report = line
                    .split(' ')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                if is_safe(&report) || is_safe_tolerated(&report) {
                    return true;
                }
                report.reverse();
                is_safe(&report) || is_safe_tolerated(&report)
            })
            .filter(|safe| safe == &true)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
