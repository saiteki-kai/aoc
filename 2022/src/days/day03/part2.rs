// https://adventofcode.com/2022/3
//
// Day 3: Rucksack Reorganization - Part 2

use std::collections::{HashMap, HashSet};

fn get_badge_priority(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    let priority_map: HashMap<char, u8> = (('a'..='z').chain('A'..='Z'))
        .zip(1..=52)
        .collect::<HashMap<char, u8>>();

    lines
        .chunks(3)
        .map(|group| get_group_badge(group.to_vec()))
        .map(|badge| *priority_map.get(&badge).unwrap_or(&0) as u32)
        .sum::<u32>()
}

fn get_group_badge(group: Vec<&str>) -> char {
    let item_types = group
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .map(HashSet::<char>::from_iter)
        .collect::<Vec<HashSet<char>>>();

    let badge: HashSet<char> = item_types[0]
        .intersection(&item_types[1])
        .cloned()
        .collect::<HashSet<char>>()
        .intersection(&item_types[2])
        .cloned()
        .collect();

    return *badge.iter().next().unwrap();
}

pub fn solve() {
    let input = include_str!("../../../inputs/day03");
    let priority = get_badge_priority(input);
    println!("badge priority: {}", priority);
}

#[cfg(test)]
mod tests {
    use super::{get_badge_priority, get_group_badge};

    #[test]
    fn test_group_badge() {
        let group1: Vec<&str> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];

        let group2: Vec<&str> = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        let badge1 = get_group_badge(group1);
        let badge2 = get_group_badge(group2);

        assert_eq!(badge1, 'r');
        assert_eq!(badge2, 'Z');
    }

    #[test]
    fn test_badge_priority() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let badge_priority = get_badge_priority(input);
        assert_eq!(badge_priority, 70);
    }
}
