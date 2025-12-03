pub fn parse(s: String) -> Vec<Vec<u32>> {
    s.trim()
        .lines()
        .map(|bank| {
            bank.chars()
                .map(|c| c.to_digit(10).unwrap_or_else(|| panic!("found non digit in bank: {}", c)))
                .collect::<Vec<u32>>()
        })
        .collect()
}

mod part1 {
    use super::*;

    pub fn largest_joltage(v: &Vec<u32>) -> u32 {
        let (first_digit, first_index) = max_i(&v[..v.len()-1]);
        let second_digit = v[first_index+1..].iter().max().unwrap_or(&0);
        first_digit * 10 + second_digit
    }

    /// get max element with index
    pub fn max_i(v: &[u32]) -> (u32, usize) {
        let mut largest = 0;
        let mut largest_i = 0;

        v.iter()
            .enumerate()
            .for_each(|(i, &n)| {
                if n > largest {
                    largest = n;
                    largest_i = i;
                }
            });

        (largest, largest_i)
    }

    pub fn solve(s: String) -> u32 {
        parse(s)
            .iter()
            .map(|x| largest_joltage(x))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::read::*;

    #[test]
    fn largest_joltage_test() {
        assert_eq!(
            part1::largest_joltage(&vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1]),
            98
        );

        assert_eq!(
            part1::largest_joltage(&vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]),
            92
        );

        assert_eq!(
            part1::largest_joltage(&vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8]),
            78
        );
    }

    #[test]
    fn part1_example() {
        let input = read_day_example("3").unwrap();
        let result = part1::solve(input);
        assert_eq!(result, 357);
    }

    #[test]
    fn part1_real() {
        let input = read_day("3").unwrap();
        let result = part1::solve(input);
        assert_eq!(result, 17166);
    }
}
