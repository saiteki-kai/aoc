// https://adventofcode.com/2022/day/5
//
// Day 5: Supply Stacks - Part 1

use std::collections::HashMap;

use regex::Regex;

pub fn solve() {
    let input_text = include_str!("../../../inputs/day05");

    let message = get_message(input_text);
    println!("message: {}", message);
}

fn get_message(input_text: &str) -> String {
    let mut stacks_lines = Vec::<&str>::new();
    let mut lines = input_text.lines();

    // read stacks
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        } else {
            stacks_lines.push(line);
        }
    }

    // init stacks
    let mut stacks = HashMap::<String, Vec<String>>::new();

    parse_stacks(&mut stacks, stacks_lines);

    // read moves
    for line in lines {
        if let Some((quantity, from, to)) = parse_move(line) {
            move_crates(&mut stacks, String::from(from), String::from(to), quantity);
        }
    }

    get_top_crates(&mut stacks)
}

fn get_top_crates(stacks: &mut HashMap<String, Vec<String>>) -> String {
    let mut sorted_keys = stacks.keys().collect::<Vec<&String>>();
    sorted_keys.sort();

    let mut message = String::new();

    for key in sorted_keys.iter() {
        if let Some(values) = stacks.get(*key) {
            let val = values.last().unwrap();
            message += val;

            println!("stack {}: {:#?}", key, val);
        }
    }

    message
}

fn parse_stacks(stacks: &mut HashMap<String, Vec<String>>, stacks_lines: Vec<&str>) {
    let (keys, values) = stacks_lines.split_last().unwrap();

    let keys = &keys
        .split_whitespace()
        .map(|s| s.replace(['[', ']'], ""))
        .collect::<Vec<String>>();

    for key in keys {
        stacks.insert(key.clone(), Vec::<String>::new());
    }

    for stack_line in values.iter().rev() {
        for key in keys {
            let col_number: usize = key.parse().unwrap();
            let col_idx: usize = (col_number) * (3 + 1) - 3; // [X]_

            if let Some(c) = stack_line.chars().nth(col_idx) {
                let s = String::from(c);

                if s != " " {
                    stacks.get_mut(key).unwrap().push(s);
                }
            }
        }
    }
}

fn parse_move(instruction: &str) -> Option<(u8, &str, &str)> {
    let re = Regex::new(r"move\s+(?<move>\d+)\s+from\s+(?<from>\d+)\s+to\s+(?<to>\d+)").unwrap();

    if let Some(result) = re.captures(instruction) {
        let quantity: u8 = result
            .name("move")
            .map_or("", |m| m.as_str())
            .parse()
            .unwrap();
        let to: &str = result.name("to").map_or("", |m| m.as_str());
        let from: &str = result.name("from").map_or("", |m| m.as_str());

        return Some((quantity, from, to));
    }

    None
}

fn move_crates(stacks: &mut HashMap<String, Vec<String>>, from: String, to: String, quantity: u8) {
    for _ in 0..quantity {
        if let Some(c) = stacks.get_mut(&from).unwrap().pop() {
            stacks.get_mut(&to).unwrap().push(c);
        } else {
            break; // empty stack
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_stack_parse() {
        let input_stack = vec![
            "    [D]           ",
            "[N] [C]           ",
            "[Z] [M] [P]       ",
            " 1   2   3        ",
        ];

        let mut stacks = HashMap::<String, Vec<String>>::new();

        parse_stacks(&mut stacks, input_stack);

        assert_eq!(
            stacks
                .get("1")
                .unwrap()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
            vec!["Z", "N"]
        );
        assert_eq!(
            stacks
                .get("2")
                .unwrap()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
            vec!["M", "C", "D"]
        );
        assert_eq!(
            stacks
                .get("3")
                .unwrap()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
            vec!["P"]
        );
        assert_eq!(stacks.keys().len(), 3);
    }

    #[test]
    fn test_parse_moves() {
        let input_moves = [
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        assert_eq!(parse_move(input_moves[0]).unwrap(), (1, "2", "1"));
        assert_eq!(parse_move(input_moves[1]).unwrap(), (3, "1", "3"));
        assert_eq!(parse_move(input_moves[2]).unwrap(), (2, "2", "1"));
        assert_eq!(parse_move(input_moves[3]).unwrap(), (1, "1", "2"));
    }

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
        assert_eq!(stacks["3"], vec!["P", "D", "N", "Z"]);
    }

    #[test]
    fn test_message() {
        let input_text = include_str!("./tests/test_input");

        assert_eq!(get_message(input_text), "CMZ");
    }
}
