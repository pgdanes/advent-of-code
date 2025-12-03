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

/// get max element with index
pub fn max_i(v: &[u32]) -> Option<(u32, usize)> {
    if v.len() == 0 {
        return None;
    }

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

    Some((largest, largest_i))
}

pub fn solve(s: String, joltage_calc: &dyn Fn(&Vec<u32>) -> Option<u64>) -> u64 {
    parse(s)
        .iter()
        .flat_map(joltage_calc)
        .sum()
}

mod part1 {
    use super::*;

    pub fn largest_joltage(v: &Vec<u32>) -> Option<u64> {
        let first_slice = &v[..v.len()-1];
        let (first_digit, first_index) = max_i(first_slice)?;

        let second_digit = v[first_index+1..]
            .iter()
            .max()
            .unwrap_or(&0);

        Some((first_digit * 10 + second_digit) as u64) 
    }
}

mod part2 {
    use super::*;

    pub fn largest_joltage(v: &Vec<u32>) -> Option<u64> {
        let mut digits: Vec<u32> = Vec::with_capacity(12);
        let mut start_index = 0;
        let mut digits_needed: usize = 12;

        // searches by finding the max number between 
        // index of the last found digit
        // and the 
        // length of array - count of the nums needed
        //
        // so first is 0..len-12
        // then first_num_index..len-11
        // then second_num_index..len-10
        // .. and so on
        while digits_needed > 0 {
            let end_index = v.len() - digits_needed + 1;
            let slice = &v[start_index..end_index];
            let (digit, relative_index) = max_i(slice)?;

            start_index += relative_index + 1;
            digits.push(digit);
            digits_needed -= 1;
        }

        let total = digits.iter().fold(0u64, |total, &digit| {
            total * 10 + digit as u64
        });

        Some(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::read::*;

    #[test]
    fn part1_largest_joltage_test() {
        assert_eq!(
            part1::largest_joltage(&vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1]),
            Some(98)
        );

        assert_eq!(
            part1::largest_joltage(&vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]),
            Some(92)
        );

        assert_eq!(
            part1::largest_joltage(&vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8]),
            Some(78)
        );
    }

    #[test]
    fn part2_largest_joltage_test() {
        assert_eq!(
            part2::largest_joltage(&vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1]),
            Some(987654321111)
        );

        assert_eq!(
            part2::largest_joltage(&vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9]),
            Some(811111111119)
        );

        assert_eq!(
            part2::largest_joltage(&vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8]),
            Some(434234234278)
        );

        assert_eq!(
            part2::largest_joltage(&vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]),
            Some(888911112111)
        );
    }

    #[test]
    fn part1_example() {
        let input = read_day_example("3").unwrap();
        let result = solve(input, &part1::largest_joltage);
        assert_eq!(result, 357);
    }

    #[test]
    fn part1_real() {
        let input = read_day("3").unwrap();
        let result = solve(input, &part1::largest_joltage);
        assert_eq!(result, 17166);
    }

    #[test]
    fn part2_example() {
        let input = read_day_example("3").unwrap();
        let result = solve(input, &part2::largest_joltage);
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn part2_real() {
        let input = read_day("3").unwrap();
        let result = solve(input, &part2::largest_joltage);
        assert_eq!(result, 169077317650774);
    }
}
