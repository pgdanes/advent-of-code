mod part1 {
    pub fn solve(s: String) -> i64 {
        return 0i64;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::read::*;

    #[test]
    fn example_part1() {
        let input = read_day_example("1_2024").unwrap();
        let result = part1::solve(input);

        assert_eq!(result, 11)
    }
}
