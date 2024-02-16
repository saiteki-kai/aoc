// https://adventofcode.com/2022/7
//
// Day 7: No Space Left On Device - Part 1

use std::{cell::RefCell, rc::Rc};

use super::shared::{build_tree, parse_file, Node};

pub fn solve() {
    let text = include_str!("../../../inputs/day07");
    let lines = parse_file(text);
    let tree = build_tree(lines);

    let total = get_total_size(tree);
    println!("{}", total);
}

pub fn get_total_size(tree: Rc<RefCell<Node>>) -> usize {
    let mut total = 0;

    for child in tree.borrow().children.values() {
        if child.borrow().is_file() {
            continue;
        }

        let folder_size = child.borrow().folder_size();

        if folder_size <= 100_000 {
            total += folder_size;
        }

        total += get_total_size(child.to_owned());
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let text = include_str!("./tests/test_input");
        let lines = parse_file(text);
        let tree = build_tree(lines);

        assert_eq!(tree.borrow().folder_size(), 48381165);
        assert_eq!(get_total_size(tree), 95437);
    }
}
