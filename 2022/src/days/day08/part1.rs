// https://adventofcode.com/2022/day/8
//
// Day 8: Treetop Tree House - Part 1

use super::shared::{parse_grid, Grid};

pub fn solve() {
    let input_text = include_str!("../../../inputs/day08");

    let grid = parse_grid(input_text);
    grid.display();

    let count = count_visibles(grid);
    println!("{}", count);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible() {
        let input_text = include_str!("./tests/test_input");

        let grid: Grid = parse_grid(input_text);
        assert_eq!(count_visibles(grid), 21);
    }
}
