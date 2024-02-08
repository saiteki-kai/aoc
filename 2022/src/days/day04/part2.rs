// https://adventofcode.com/2022/4
//
//  Day 4: Camp Cleanup - Part 2

use super::shared::{find_overlap, OverlapType};

pub fn solve() {
    let input_text = include_str!("../../../inputs/day04");
    let pairs = input_text.lines().collect::<Vec<&str>>();

    let overlaps = pairs
        .iter()
        .filter(|pair| find_overlap(pair, OverlapType::Partial))
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

        assert!(find_overlap("2-8,3-7", OverlapType::Partial));
        assert!(!find_overlap("2-3,4-5", OverlapType::Partial));

        let overlaps = pairs
            .iter()
            .filter(|pair| find_overlap(pair, OverlapType::Partial))
            .count();

        assert_eq!(overlaps, 4);
    }
}
