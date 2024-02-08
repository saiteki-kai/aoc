// https://adventofcode.com/2022/4
//
//  Day 4: Camp Cleanup - Part 1

use std::collections::HashSet;

pub fn solve() {
    let input_text = include_str!("../../../inputs/day04");
    let pairs = input_text.lines().collect::<Vec<&str>>();

    let overlaps = pairs.iter().filter(|pair| find_overlap(pair)).count();
    println!("#overlaps: {}", overlaps);
}

fn get_sections(minmax: &str) -> HashSet<u8> {
    let mm = minmax.split('-').collect::<Vec<&str>>();

    let min: u8 = mm[0].parse().unwrap();
    let max: u8 = mm[1].parse().unwrap();
    let range = min..=max;

    range.collect::<HashSet<u8>>()
}

fn find_overlap(pair: &str) -> bool {
    println!("{}", pair);
    let elf_sections = pair.split(',').collect::<Vec<&str>>();

    let elf1_sections = get_sections(elf_sections[0]);
    let elf2_sections = get_sections(elf_sections[1]);

    elf1_sections.is_subset(&elf2_sections) || elf2_sections.is_subset(&elf1_sections)
}

#[cfg(test)]
mod tests {
    use super::find_overlap;

    #[test]
    fn test_pairs() {
        let pairs = [
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ];

        let overlaps = pairs.iter().filter(|pair| find_overlap(pair)).count();

        assert_eq!(overlaps, 2);
    }
}
