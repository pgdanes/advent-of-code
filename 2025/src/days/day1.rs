#[derive(Debug)]
pub enum Rotation {
    Left(i32),
    Right(i32)
}

pub fn parse(s: String) -> Vec<Rotation> {
    let r: Vec<Rotation> = s.lines()
        .map(|line| {
            let rotation_char = &line[..1];
            let num_chars = &line[1..];

            let num = num_chars.parse::<i32>().expect("couldn't parse nums of line");
            match rotation_char {
                "L" => Rotation::Left(num),
                "R" => Rotation::Right(num),
                _ => panic!("unexpected dial rotation character {rotation_char}")
            }
        }).collect();

    r
}

mod part1 {
    use super::*;

    pub fn solve(s: String) -> i32 {
        let mut number = 50;
        let mut zero_count = 0;

        let rotations = parse(s);

        for rotation in rotations {
            let mut next: i32;            

            match rotation {
                Rotation::Left(l) => {
                    next = number - (l % 100);
                },
                Rotation::Right(r) => {
                    next = number + (r % 100);
                }
            }

            if next < 0 {
                next += 100;
            } else if next > 99 {
                next %= 100;
            }


            println!("{number} -{rotation:?}-> {next}");
            number = next;
            if number == 0 {
                zero_count += 1;
            }
        }

        zero_count
    }
}

mod part2 {
    use super::*;

    pub fn solve(s: String, start: i32) -> i32 {
        let mut num = start;
        let mut zero_count = 0;

        let rotations = parse(s);

        for rotation in rotations {
            let (amount, delta) = match rotation {
                Rotation::Left(n) => (n, -1),
                Rotation::Right(n) => (n, 1)
            };

            for _ in 0..amount {
                num += delta;
                num %= 100;

                if num == 0 {
                    zero_count += 1
                }
            }
        }

        zero_count
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::read::*;

    #[test]
    fn part1_example() {
        let input = read_day_example("1").unwrap();
        let result = part1::solve(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn part1_test1() {
        let input = "L45\nL10".to_string();
        let result = part1::solve(input);
        assert_eq!(result, 0);

        let input = "L45\nL10\nR5".to_string();
        let result = part1::solve(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn part1_real() {
        let input = read_day("1").unwrap();
        let result = part1::solve(input);
        assert_eq!(result, 1034);
    }

    #[test]
    fn part2_example() {
        let input = read_day_example("1").unwrap();
        let result = part2::solve(input, 50);
        assert_eq!(result, 6);
    }

    #[test]
    fn part2_real() {
        let input = read_day("1").unwrap();
        let result = part2::solve(input, 50);
        assert_eq!(result, 6166);
    }

    #[test]
    fn part2_test_land_on_zero() {
        let inputs = vec!(
            ("R94", 0),
            ("R95", 1),
            ("R195", 2),
            ("L4", 0),
            ("L5", 1),
            ("L105", 2),
        );

        for (input, expected) in inputs {
            assert_eq!(part2::solve(input.to_string(), 5), expected, "test: {input:?}")
        }
    }

    #[test]
    fn part2_test_from_zero() {
        let inputs = vec!(
            ("L5", 0),
            ("R5", 0),
        );

        for (input, expected) in inputs {
            assert_eq!(part2::solve(input.to_string(), 0), expected, "test: {input:?}")
        }
    }
}
