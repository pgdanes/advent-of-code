pub struct IdRange {
    start: i64,
    end: i64
}

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

mod part1 {
    use super::*;

    pub fn solve(s: String) -> i64 {
        let ranges = parse(s);

        let mut total = 0;

        ranges
            .iter()
            .for_each(|range| {
                for i in range.start..=range.end {
                    if is_invalid(&i.to_string()) {
                        total += i
                    }
                }
            });

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::read::*;

    #[test]
    fn is_invalid_test() {
        assert_eq!(is_invalid("11"), true);
        assert_eq!(is_invalid("22"), true);
        assert_eq!(is_invalid("99"), true);
        assert_eq!(is_invalid("1010"), true);
        assert_eq!(is_invalid("222222"), true);
        assert_eq!(is_invalid("123123"), true);
        assert_eq!(is_invalid("1188511885"), true);
        assert_eq!(is_invalid("38593859"), true);

        assert_eq!(is_invalid("10"), false);
        assert_eq!(is_invalid("101"), false);
        assert_eq!(is_invalid("1011"), false);
        assert_eq!(is_invalid("123"), false);
    }

    #[test]
    fn part1_example() {
        let input = read_day_example("2").unwrap();
        let result = part1::solve(input);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn part1_real() {
        let input = read_day("2").unwrap();
        let result = part1::solve(input);
        assert_eq!(result, 38437576669);
    }
}
