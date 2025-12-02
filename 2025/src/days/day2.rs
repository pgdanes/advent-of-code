use std::{ops::Div, time::Instant};

pub struct IdRange {
    start: i64,
    end: i64
}


pub fn parse(s: String) -> Vec<IdRange> {
    s.trim()
        .split(",")
        .map(|range| {
            let (left, right) = range.split_once("-").expect("failed to split id range");
            IdRange {
                start: left.parse::<i64>().unwrap_or_else(|_| panic!("failed to parse left num {left:?}")),
                end: right.parse::<i64>().unwrap_or_else(|_| panic!("failed to parse right num {right:?}"))
            }
        })
        .collect()
}

pub fn solve(s: String, invalid_fn: &dyn Fn(&str) -> bool) -> i64 {
    let ranges = parse(s);
    let mut total = 0;
    let mut iterations = 0;

    let now = Instant::now();
    ranges
        .iter()
        .for_each(|range| {
            for i in range.start..=range.end {
                iterations += 1;
                if invalid_fn(&i.to_string()) {
                    total += i
                }
            }
        });

    let avg = now.elapsed().div(iterations);

    println!("range count: {}", ranges.len());
    println!("iteration count: {iterations}");
    println!("avg iteration time: {avg:?}");
    println!("total time: {:?}", now.elapsed());

    total
}

mod part1 {
    pub fn is_invalid(id: &str) -> bool {
        let length = id.len();
        if length % 2 != 0 {
            return false;
        }

        let mid = length/2;
        let left = &id[..mid];
        let right = &id[mid..];
        left == right
    }
}

mod part2 {
    use std::{cmp::min};

    pub fn is_invalid(id: &str) -> bool {
        let length = id.len();
        if length <= 1 {
            return false;
        }
        let mid = length/2;

        for part_length in 1..=mid {
            if length % part_length != 0 {
                continue;
            }

            let mut start = 0;
            let end = min(start+part_length, length);
            let first = &id[start..end];
            let mut all_match = true;

            while start < length {
                let end = min(start+part_length, length);
                let part = &id[start..end];
                all_match &= part == first;
                start += part_length;
            }

            if all_match {
                return true;
            }
        }
 
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::read::*;

    #[test]
    fn part1_example() {
        let input = read_day_example("2").unwrap();
        let result = solve(input, &part1::is_invalid);
        assert_eq!(result, 1227775554);
    }

    #[test]
    #[ignore = "for quicker runs"]
    fn part1_real() {
        let input = read_day("2").unwrap();
        let result = solve(input, &part1::is_invalid);
        assert_eq!(result, 38437576669);
    }

    #[test]
    fn part2_example() {
        let input = read_day_example("2").unwrap();
        let result = solve(input, &part2::is_invalid);
        assert_eq!(result, 4174379265);
    }

    #[test]
    #[ignore = "for quicker runs"]
    fn part2_real() {
        let input = read_day("2").unwrap();
        let result = solve(input, &part2::is_invalid);
        assert_eq!(result, 49046150754);
    }

    #[test]
    fn part1_is_invalid_test() {
        assert_eq!(part1::is_invalid("11"), true);
        assert_eq!(part1::is_invalid("22"), true);
        assert_eq!(part1::is_invalid("99"), true);
        assert_eq!(part1::is_invalid("1010"), true);
        assert_eq!(part1::is_invalid("222222"), true);
        assert_eq!(part1::is_invalid("123123"), true);
        assert_eq!(part1::is_invalid("1188511885"), true);
        assert_eq!(part1::is_invalid("38593859"), true);

        assert_eq!(part1::is_invalid("10"), false);
        assert_eq!(part1::is_invalid("101"), false);
        assert_eq!(part1::is_invalid("1011"), false);
        assert_eq!(part1::is_invalid("123"), false);
    }

    #[test]
    fn part2_is_invalid_test() {
        assert_eq!(part2::is_invalid("11"), true);
        assert_eq!(part2::is_invalid("22"), true);
        assert_eq!(part2::is_invalid("99"), true);
        assert_eq!(part2::is_invalid("1010"), true);
        assert_eq!(part2::is_invalid("222222"), true);

        assert_eq!(part2::is_invalid("10"), false);
        assert_eq!(part2::is_invalid("101"), false);
        assert_eq!(part2::is_invalid("1011"), false);
        assert_eq!(part2::is_invalid("123"), false);

        // new ones
        assert_eq!(part2::is_invalid("111"), true);
        assert_eq!(part2::is_invalid("11111"), true);
        assert_eq!(part2::is_invalid("121212"), true);
        assert_eq!(part2::is_invalid("446446"), true);
        assert_eq!(part2::is_invalid("38593859"), true);
        assert_eq!(part2::is_invalid("2121212121"), true);
        assert_eq!(part2::is_invalid("2121212118"), false);
    }
}
