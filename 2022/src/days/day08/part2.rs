// https://adventofcode.com/2022/8
//
// Day 8: Treetop Tree House - Part 2

use super::shared::{parse_grid, Direction, Grid, DIRECTIONS};

pub fn solve() {
    let input_text = include_str!("../../../inputs/day08");

    let grid = parse_grid(input_text);
    grid.display();

    let count = find_best_scenic_score(grid);
    println!("{}", count);
}

fn find_best_scenic_score(grid: Grid) -> usize {
    let mut m = 0;

    for row in 1..(grid.rows - 1) {
        for col in 1..(grid.cols - 1) {
            let v = grid.scenic_score(row, col);

            if v > m {
                m = v;
            }
        }
    }

    m
}

impl Grid {
    fn viewing_distance(&self, i: usize, j: usize, direction: Direction) -> usize {
        if let Some(curr_height) = self.get(i, j) {
            let x = self.adjacent_k_trees(i, j, direction);

            let mut count = 0;

            for y in x {
                if y < curr_height {
                    count += 1;
                    continue;
                }

                if y >= curr_height {
                    count += 1;
                    break;
                }
            }

            return count;
        }

        0
    }

    fn scenic_score(&self, i: usize, j: usize) -> usize {
        DIRECTIONS
            .iter()
            .map(|&d| self.viewing_distance(i, j, d))
            .product::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day08::shared::parse_grid;

    use super::*;
    use test_case::test_case;

    #[test_case(1, 2, Direction::Top, 1)]
    #[test_case(1, 2, Direction::Right, 2)]
    #[test_case(1, 2, Direction::Left, 1)]
    #[test_case(1, 2, Direction::Bottom, 2)]
    #[test_case(3, 2, Direction::Top, 2)]
    #[test_case(3, 2, Direction::Right, 2)]
    #[test_case(3, 2, Direction::Left, 2)]
    #[test_case(3, 2, Direction::Bottom, 1)]
    fn test_viewing_distance(i: usize, j: usize, d: Direction, distance: usize) {
        let input_text = include_str!("./tests/test_input");
        let grid: Grid = parse_grid(input_text);

        assert_eq!(grid.viewing_distance(i, j, d), distance);
    }

    #[test_case(1, 2, 4)]
    #[test_case(3, 2, 8)]
    fn test_scenic_score(i: usize, j: usize, score: usize) {
        let input_text = include_str!("./tests/test_input");
        let grid: Grid = parse_grid(input_text);

        assert_eq!(grid.scenic_score(i, j), score);
    }
}
