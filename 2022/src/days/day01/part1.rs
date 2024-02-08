// https://adventofcode.com/2022/1
//
// Day 1: Calorie Counting - Part 1

use std::fs;
use std::path::Path;
use std::str;

pub fn get_input() -> String {
    let filepath: &Path = Path::new("./inputs/day01");
    let file: String = fs::read_to_string(filepath).expect("Error while reading the file.");

    file
}

fn solve_v1() {
    let file = get_input();
    let x: str::Split<'_, char> = file.trim().split('\n');

    let mut max: u32 = 0;
    let mut curr: u32 = 0;

    for y in x {
        if y.trim() == "" {
            if curr > max {
                max = curr;
                curr = 0;
            }
        } else {
            curr += y.trim().parse::<u32>().expect("NaN");
        }
    }

    println!("{max}");
}

fn solve_v2() {
    let file = include_str!("../../../inputs/day01");
    let elves = file.trim().split("\n\n");

    let mut max_calories: u32 = 0;
    let mut max_i: usize = 0;

    for (i, elf) in elves.enumerate() {
        let sum = elf
            .split('\n')
            .map(|x| x.trim().parse::<u32>().unwrap())
            .sum::<u32>();

        if sum > max_calories {
            max_calories = sum;
            max_i = i;
        }
    }

    println!(
        "The elf number {} have a total of {} calories.",
        max_i + 1,
        max_calories
    );
}

pub fn solve(version: Option<u8>) {
    match version {
        Some(1) => solve_v1(),
        Some(2) => solve_v2(),
        _ => println!("Invalid version. Avaliable versions: 1, 2"),
    }
}
