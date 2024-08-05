// https://adventofcode.com/2022/day/4
//
//  Day 4: Camp Cleanup - Part 1

use super::shared::{find_overlap, OverlapType};

pub fn solve() {
    let input_text = include_str!("../../../inputs/day04");
    let pairs = input_text.lines().collect::<Vec<&str>>();

    let overlaps = pairs
        .iter()
        .filter(|pair| find_overlap(pair, OverlapType::Full))
        .count();

    println!("#overlaps: {}", overlaps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs() {
        let pairs = [
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ];

        let overlaps = pairs
            .iter()
            .filter(|pair| find_overlap(pair, OverlapType::Full))
            .count();

        assert_eq!(overlaps, 2);
    }
}
