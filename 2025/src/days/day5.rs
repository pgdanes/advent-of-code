use std::cmp::max;

type Id = usize;

pub struct Range {
    start: Id,
    end: Id  
}

pub fn parse(s: String) -> (Vec<Range>, Vec<Id>) {
    let normalised = s.replace("\r\n", "\n");

    let (ranges_str, ids_str) = normalised
        .trim()
        .split_once("\n\n")
        .expect("bad input, didn't include blank line between sections");

    let ranges: Vec<Range> = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").expect("bad range");
            Range {
                start: start.parse::<Id>().expect("bad input for start of range"),
                end: end.parse::<Id>().expect("bad input for start of range")
            }
        })
        .collect();

    let ids: Vec<Id> = ids_str
        .lines()
        .map(|line| {
            line.parse::<Id>().expect("bad ingredient id")
        })
        .collect();

    (ranges, ids)
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_by(|l, r| {
        l.start.cmp(&r.start)
    });

    let mut merged_ranges: Vec<Range> = Vec::with_capacity(ranges.capacity());
    for range in ranges {
        match merged_ranges.last_mut() {
            Some(last) => {
                if last.end < range.start {
                    merged_ranges.push(range);
                } else {
                   last.end = max(last.end, range.end)
                }
            }
            None => merged_ranges.push(range)
        }
    }

    merged_ranges
}


mod part1 {
    use super::*;

    pub fn solve(s: String) -> usize {
        let (ranges, ids) = parse(s);
        let merged_ranges = merge_ranges(ranges);

        ids
            .iter()
            .filter(|&id| {
                merged_ranges
                    .iter()
                    .any(|range| {
                        *id >= range.start && *id <= range.end
                    })
            })
            .count()
    }
}

mod part2 {
    use super::*;

    pub fn solve(s: String) -> usize {
        let (ranges, _) = parse(s);
        let merged_ranges = merge_ranges(ranges);

        merged_ranges
            .iter()
            .map(|range| {
                range.end+1 - range.start
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::read::*;

    #[test]
    fn part1_example() {
        let input = read_day_example("5").unwrap();
        let result = part1::solve(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn part1_real() {
        let input = read_day("5").unwrap();
        let result = part1::solve(input);
        assert_eq!(result, 640);
    }

    #[test]
    fn part2_example() {
        let input = read_day_example("5").unwrap();
        let result = part2::solve(input);
        assert_eq!(result, 14);
    }

    #[test]
    fn part2_real() {
        let input = read_day("5").unwrap();
        let result = part2::solve(input);
        assert_eq!(result, 365804144481581);
    }
}
