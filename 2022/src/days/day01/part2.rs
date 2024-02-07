// https://adventofcode.com/2022/1
//
// Day 1: Calorie Counting - Part 2

use super::part1::get_input;

fn get_elf_calories(elf: &str) -> u32 {
    let elf_calories = elf
        .split("\n")
        .map(|x| x.trim().parse::<u32>().unwrap())
        .sum::<u32>();

    return elf_calories;
}

fn part2(elves: std::str::Split<'_, &str>) -> u32 {
    let mut calories = elves.map(get_elf_calories).collect::<Vec<u32>>();
    calories.sort_unstable();
    calories.reverse();

    let total = calories[0] + calories[1] + calories[2];
    println!("{total}");

    total
}

pub fn solve() {
    let file = get_input();
    let elves = file.trim().split("\n\n");

    part2(elves);
}
