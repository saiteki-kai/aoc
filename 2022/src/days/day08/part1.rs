// https://adventofcode.com/2022/8
//
// Day 8: Treetop Tree House - Part 1

use std::vec;

pub fn solve() {
    let input_text = include_str!("../../../inputs/day08");

    let grid = parse_grid(input_text);
    grid.display();

    let count = count_visibles(grid);
    println!("{}", count);
}

fn parse_grid(input_text: &str) -> Grid {
    let width = input_text.lines().next().unwrap().len();
    let height = input_text.lines().count();

    let mut grid = Grid::new(height, width);

    let rows = input_text.lines();
    for row in rows {
        let cols = row.chars().map(|c| c.to_string());

        for col in cols {
            let value = col.parse::<u8>().unwrap();
            grid.elements.push(value);
        }
    }

    grid
}

fn count_visibles(grid: Grid) -> u64 {
    let mut interior_total: u64 = 0;

    for row in 1..(grid.rows - 1) {
        for col in 1..(grid.cols - 1) {
            if grid.visible(row, col) {
                interior_total += 1;
            }
        }
    }

    let edge_total = 2 * (grid.rows + grid.cols) - 4;

    interior_total + edge_total as u64
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

fn get_direction_value(direction: Direction) -> (i8, i8) {
    match direction {
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Top => (0, -1),
        Direction::Bottom => (0, 1),
    }
}

static DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Right,
    Direction::Top,
    Direction::Bottom,
];

#[derive(Default)]
struct Grid {
    rows: usize,
    cols: usize,
    elements: Vec<u8>,
}

impl Grid {
    fn new(n_rows: usize, n_cols: usize) -> Grid {
        Grid {
            rows: n_rows,
            cols: n_cols,
            elements: vec![],
        }
    }

    fn get(&self, i: usize, j: usize) -> Option<u8> {
        if i >= self.rows || j >= self.cols {
            return None;
        }

        let idx: usize = j * self.rows + i;
        Some(self.elements[idx])
    }

    fn adjacent_k_trees(&self, i: usize, j: usize, direction: Direction) -> Vec<u8> {
        let max_length = self.cols.max(self.rows);

        let (x, y) = get_direction_value(direction);

        (1..max_length)
            .map_while(|k| {
                self.get(
                    i.checked_add_signed((k as isize) * (x as isize))?,
                    j.checked_add_signed((k as isize) * (y as isize))?,
                )
            })
            .collect::<Vec<u8>>()
    }

    fn visible(&self, i: usize, j: usize) -> bool {
        let value = self.get(i, j).unwrap();

        DIRECTIONS.iter().any(|&dir| {
            self.adjacent_k_trees(i, j, dir)
                .into_iter()
                .all(|v| v < value)
        })
    }

    fn display(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{}", self.get(i, j).unwrap());
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day08::part1::{count_visibles, Direction};

    use super::{parse_grid, Grid};

    #[test]
    fn test_construction() {
        let input_text = include_str!("./tests/test_input");

        let grid: Grid = parse_grid(input_text);
        grid.display();
        assert_eq!(grid.rows, 5);
        assert_eq!(grid.cols, 5);

        assert_eq!(grid.get(1, 2).unwrap(), 5);
        assert_eq!(grid.get(3, 3).unwrap(), 4);

        assert_eq!(grid.adjacent_k_trees(2, 3, Direction::Right).len(), 2);
        assert_eq!(grid.adjacent_k_trees(2, 3, Direction::Bottom).len(), 1);
        assert_eq!(grid.adjacent_k_trees(2, 3, Direction::Left).len(), 2);
        assert_eq!(grid.adjacent_k_trees(2, 3, Direction::Top).len(), 3);

        assert_eq!(
            grid.adjacent_k_trees(2, 3, Direction::Bottom)
                .into_iter()
                .max()
                .unwrap(),
            3
        );
        assert_eq!(
            grid.adjacent_k_trees(2, 3, Direction::Top)
                .into_iter()
                .max()
                .unwrap(),
            5
        );
    }

    #[test]
    fn test_visible() {
        let input_text = include_str!("./tests/test_input");

        let grid: Grid = parse_grid(input_text);
        assert_eq!(count_visibles(grid), 21);
    }
}
