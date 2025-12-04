use std::collections::HashSet;

type Grid = HashSet<(isize, isize)>;

pub fn parse(s: String) -> Grid {
    let mut grid = HashSet::new();

    s.trim()
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .for_each(|(x, char)| {
                    if char == '@' {
                        grid.insert((x as isize, y as isize));
                    }
                });
        });

    grid
}

static ADJACENCIES: &'static [(isize, isize)] = &[
    (-1,-1),
    (-1,0),
    (-1,1),
    (0,-1),
    (0,1),
    (1,-1),
    (1,0),
    (1,1),
];

pub fn count_neighbours(grid: &Grid, coord: (isize, isize)) -> u32 {
    ADJACENCIES
        .iter()
        .map(|adjacency| {
            let x = adjacency.0 + coord.0;
            let y = adjacency.1 + coord.1;
            (x, y)
        })
        .fold(0u32, |acc, coord| {
            acc + if grid.contains(&coord) { 1 } else { 0 } 
        })
}

mod part1 {
    use super::*;

    pub fn solve(s: String) -> u32 {
        let grid = parse(s);

        grid.iter()
            .map(|&paper_coord| {
                if count_neighbours(&grid, paper_coord) < 4 { 1 } else { 0 }
            })
            .sum()
    }
}

mod part2 {
    use super::*;

    pub fn solve(s: String) -> u32 {
        let mut grid = parse(s);

        let mut removed = 1;
        let mut total: u32 = 0;

        while removed > 0 {
            let found: Vec<(isize, isize)> = grid
                .iter()
                .flat_map(|&paper_coord| {
                    let count = count_neighbours(&grid, paper_coord) ;
                    if count < 4 { Some(paper_coord.to_owned()) } else { None }
                })
                .collect();

            removed = found.iter().count() as u32;
            total += removed;

            for paper_coord in found {
                grid.remove(&paper_coord);
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::read::*;

    #[test]
    fn parse_test() {
        let input = "..@@.@@@@.\n@@@.@.@.@@".to_string();
        let result = parse(input);
        println!("{:?}", result);

        assert_eq!(result.get(&(2,0)), Some(&(2,0)));
        assert_eq!(result.get(&(0,0)), None);
        assert_eq!(result.get(&(0,1)), Some(&(0,1)));
        assert_eq!(result.get(&(3,1)), None);
    }

    #[test]
    fn part1_example() {
        let input = read_day_example("4").unwrap();
        let result = part1::solve(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn part1_real() {
        let input = read_day("4").unwrap();
        let result = part1::solve(input);
        assert_eq!(result, 1478);
    }

    #[test]
    fn part2_example() {
        let input = read_day_example("4").unwrap();
        let result = part2::solve(input);
        assert_eq!(result, 43);
    }

    #[test]
    fn part2_real() {
        let input = read_day("4").unwrap();
        let result = part2::solve(input);
        assert_eq!(result, 9120);
    }
}
