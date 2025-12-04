mod part1 {
    use std::{error::Error, iter::zip, num::ParseIntError};

    pub fn solve(s: String) -> Result<i32, Box<dyn Error>> {
        let parsed = parse(&s)?;
        let (mut left, mut right) = pivot(parsed);

        left.sort();
        right.sort();

        let result: i32 = zip(left, right)
            .map(|(l, r)| (l - r).abs())
            .sum();

        Ok(result)
    }

    fn parse(s: &str) -> Result<Vec<(i32, i32)>, ParseIntError> {
        s.lines()
            .map(|line| {
                let (l, r) = line
                    .split_once("   ")
                    .ok_or_else(|| "Failed to split line"
                        .to_string()
                        .parse::<i32>()
                        .unwrap_err())?;

                    let left = l.parse()?;
                    let right = r.parse()?;
                    Ok((left, right))
            })
            .collect()
    }

    fn pivot(vec: Vec<(i32, i32)>) -> (Vec<i32>, Vec<i32>) {
        let first = vec.iter().map(|f| f.0).collect();
        let second = vec.iter().map(|f| f.1).collect();
        (first, second)
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::days::read::*;
//
//     #[test]
//     fn example_part1() {
//         let input = read_day_example("1_2024").unwrap();
//         let result = part1::solve(input);
//
//         assert!(result.is_ok());
//         assert_eq!(result.unwrap(), 11);
//     }
//
//     #[test]
//     fn real_part1() {
//         let input = read_day("1_2024").unwrap();
//         let result = part1::solve(input);
//
//         assert!(result.is_ok());
//         assert_eq!(result.unwrap(), 2166959);
//     }
// }
