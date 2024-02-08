// https://adventofcode.com/2022/3
//
// Day 3: Rucksack Reorganization - Part 1

use std::collections::HashMap;
use std::collections::HashSet;

fn get_priority(p1: Vec<u8>, p2: Vec<u8>) -> u32 {
    let p1 = HashSet::<u8>::from_iter(p1);
    let p2 = HashSet::<u8>::from_iter(p2);

    let p3 = p1.intersection(&p2).cloned().collect::<Vec<u8>>();

    p3.into_iter().map(|x| x as u32).sum::<u32>()
}

fn get_total_priority(input: &str) -> u32 {
    let lines = input.lines();

    let priority_map = (('a'..='z').chain('A'..='Z'))
        .zip(1..=52)
        .collect::<HashMap<char, u8>>();

    let mut total = 0;

    for line in lines {
        let index = line.len() / 2;

        let p1 = line[..index]
            .chars()
            .map(|x| priority_map[&x])
            .collect::<Vec<u8>>();
        let p2 = line[index..]
            .chars()
            .map(|x| priority_map[&x])
            .collect::<Vec<u8>>();

        let priority = get_priority(p1, p2);

        total += priority;
    }

    total
}

pub fn solve() {
    let input = include_str!("../../../inputs/day03");
    let priority = get_total_priority(input);
    println!("priority: {}", priority);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let total = get_total_priority(input);
        assert_eq!(total, 157);
    }
}
