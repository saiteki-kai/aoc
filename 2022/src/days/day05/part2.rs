// https://adventofcode.com/2022/day/5
//
// Day 5: Supply Stacks - Part 2

use std::collections::HashMap;

use super::shared::*;

pub fn solve() {
    let input_text = include_str!("../../../inputs/day05");

    let message = get_message(input_text, move_crates);
    println!("message: {}", message);
}

fn move_crates(stacks: &mut HashMap<String, Vec<String>>, from: String, to: String, quantity: u8) {
    let stack_from = stacks.get_mut(&from).unwrap();

    let idx = stack_from.len() - quantity as usize;
    let last_crates = stack_from.split_off(idx);

    stacks.get_mut(&to).unwrap().extend(last_crates);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_apply_move() {
        let mut stacks = HashMap::<String, Vec<String>>::new();
        stacks.insert(
            "1".to_string(),
            ["Z", "N"].iter().map(|&s| s.to_string()).collect(),
        );
        stacks.insert(
            "2".to_string(),
            ["M", "C", "D"].iter().map(|&s| s.to_string()).collect(),
        );
        stacks.insert(
            "3".to_string(),
            ["P"].iter().map(|&s| s.to_string()).collect(),
        );

        move_crates(&mut stacks, "2".to_string(), "1".to_string(), 1);
        assert_eq!(stacks["1"], vec!["Z", "N", "D"]);

        move_crates(&mut stacks, "1".to_string(), "3".to_string(), 3);
        assert_eq!(stacks["3"], vec!["P", "Z", "N", "D"]);
    }

    #[test]
    fn test_message() {
        let input_text = include_str!("./tests/test_input");

        assert_eq!(get_message(input_text, move_crates), "MCD");
    }
}
