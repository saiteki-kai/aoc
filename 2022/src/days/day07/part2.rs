// https://adventofcode.com/2022/day/7
//
// Day 7: No Space Left On Device - Part 2

use std::{cell::RefCell, cmp::min, rc::Rc};

use super::shared::{build_tree, parse_file, Node};

const SPACE_AVAILABLE: usize = 70_000_000;
const SPACE_REQUIRED: usize = 30_000_000;

pub fn solve() {
    let text = include_str!("../../../inputs/day07");
    let lines = parse_file(text);
    let tree = build_tree(lines);

    let free_space = SPACE_AVAILABLE - tree.borrow().folder_size();
    let total = find_smallest_folder_size(tree, free_space);
    println!("{}", total);
}

pub fn find_smallest_folder_size(tree: Rc<RefCell<Node>>, free_space: usize) -> usize {
    let mut smallest = SPACE_AVAILABLE;

    for child in tree.borrow().children.values() {
        if child.borrow().is_file() {
            continue;
        }

        let folder_size = child.borrow().folder_size();

        if free_space + folder_size >= SPACE_REQUIRED {
            smallest = min(folder_size, smallest);
            smallest = min(smallest, find_smallest_folder_size(child.to_owned(), free_space));
        }
    }

    smallest
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
        assert_eq!(find_smallest_folder_size(tree, SPACE_AVAILABLE - 48381165), 24933642);
    }
}
