// https://adventofcode.com/2022/day/12
//
// Day 12: Hill Climbing Algorithm - Part 1

use rand::prelude::SliceRandom;

use super::heatmap::{read_heatmap, HeatMap};

pub fn solve() {
    let input = include_str!("../../../inputs/day12");
    let heatmap = read_heatmap(input);

    random_path(&heatmap);
}

fn random_path(heatmap: &HeatMap) {
    let start = heatmap.start().expect("Start not defined");

    let mut current_square = Box::new(*start);

    println!("{}", start.value);

    while !current_square.is_end() {
        let dests = heatmap.possible_destinations(&current_square);
        println!("{} possible destinations", dests.len());

        print!("{}: ", current_square.value);
        if current_square.value == 'd' {
            println!("entered!");
        } else {
            println!();
        }

        if dests.is_empty() {
            println!("No possible destinations");
            break;
        }

        // random move with probability based on elevation (more elevated = higher probability)
        current_square = Box::new(
            *dests
                .choose_weighted(&mut rand::thread_rng(), |d| d.get_elevation().unwrap_or(0))
                .unwrap(),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day12::heatmap::read_heatmap;

    use super::random_path;

    #[test]
    fn test_format() {
        let input = include_str!("./tests/test_input");
        let heatmap = read_heatmap(input);

        // check dimensions
        assert_eq!(heatmap.rows(), 5);
        assert_eq!(heatmap.cols(), 8);
        assert_eq!(heatmap.len(), 8 * 5);

        // check squares
        for square in heatmap.clone() {
            assert!(square.col < heatmap.cols());
            assert!(square.row < heatmap.rows());

            assert!(square.value.is_alphabetic());
            assert!(square.get_elevation().is_some_and(|e| (1..27).contains(&e)));
        }

        // check start and end count
        assert_eq!(heatmap.clone().filter(|s| s.is_start()).count(), 1);
        assert_eq!(heatmap.clone().filter(|s| s.is_end()).count(), 1);
    }

    #[test]
    fn test_start() {
        let input = include_str!("./tests/test_input");
        let heatmap = read_heatmap(input);

        let start = heatmap.start().expect("Start not defined");

        assert_eq!(start.row, 0);
        assert_eq!(start.col, 0);

        assert_eq!(heatmap.neighbour(start).len(), 2);
        assert_eq!(heatmap.possible_destinations(start).len(), 2);
    }

    #[test]
    fn test_random_path() {
        let input = include_str!("./tests/test_input");
        let heatmap = read_heatmap(input);

        random_path(&heatmap);
    }
}
