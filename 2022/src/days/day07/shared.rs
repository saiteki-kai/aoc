use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Tree structure inspired by:
// https://fasterthanli.me/series/advent-of-code-2022/part-7

#[derive(Default)]
pub struct Node {
    pub size: usize,
    pub children: HashMap<String, Rc<RefCell<Node>>>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn folder_size(&self) -> usize {
        self.children
            .values()
            .map(|child| child.borrow().folder_size())
            .sum::<usize>()
            + self.size
    }

    pub fn is_file(&self) -> bool {
        self.children.is_empty()
    }
}

pub fn build_tree(lines: Vec<Line>) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::default()));
    let mut curr_node = root.clone();

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Cd(path) => match path.as_str() {
                    "/" => {}
                    ".." => {
                        let parent = curr_node.borrow().parent.clone().unwrap();
                        curr_node = parent;
                    }
                    _ => {
                        let child = curr_node
                            .borrow_mut()
                            .children
                            .entry(path)
                            .or_default()
                            .clone();
                        curr_node = child;
                    }
                },
                Command::Ls => {}
                _ => {}
            },
            Line::Output(out) => match out {
                Output::File(size, name) => {
                    let entry = curr_node
                        .borrow_mut()
                        .children
                        .entry(name)
                        .or_default()
                        .clone();
                    entry.borrow_mut().size = size;
                    entry.borrow_mut().parent = Some(curr_node.clone());
                }
                Output::Directory(name) => {
                    let entry = curr_node
                        .borrow_mut()
                        .children
                        .entry(name)
                        .or_default()
                        .clone();
                    entry.borrow_mut().parent = Some(curr_node.clone());
                }
            },
        }
    }

    root
}

pub fn parse_file(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Line {
    if let Some(text) = line.strip_prefix("$ ") {
        let command = parse_command(text);

        Line::Command(command)
    } else {
        let output = parse_output(line);

        Line::Output(output)
    }
}

fn parse_output(text: &str) -> Output {
    let (size_or_dir, name) = text.split_once(' ').unwrap();

    if size_or_dir == "dir" {
        Output::Directory(name.to_string())
    } else {
        Output::File(size_or_dir.parse::<usize>().unwrap(), name.to_string())
    }
}

fn parse_command(text: &str) -> Command {
    let args = text.split_whitespace().collect::<Vec<&str>>();
    let command = args[0];

    match command {
        "ls" => Command::Ls,
        "cd" => {
            if let Some(directory) = args.get(1) {
                Command::Cd(directory.to_string())
            } else {
                Command::Invalid
            }
        }
        _ => Command::Undefined,
    }
}

#[derive(Debug)]
pub enum Command {
    Cd(String),
    Ls,
    Undefined,
    Invalid,
}

pub enum Output {
    File(usize, String),
    Directory(String),
}

pub enum Line {
    Command(Command),
    Output(Output),
}

#[cfg(test)]
mod tests {
    use super::{build_tree, parse_file};

    #[test]
    fn test_tree() {
        let example = include_str!("./tests/test_input");
        let lines = parse_file(example);
        let tree = build_tree(lines);

        let root = tree.borrow();
        assert_eq!(root.children.len(), 4);
        assert_eq!(root.size, 0);
        assert!(!root.is_file());
        assert_eq!(root.folder_size(), 48381165);

        let folder_a = root.children.get("a").unwrap();
        assert_eq!(folder_a.borrow().folder_size(), 94853);

        let file_b = root.children.get("b.txt").unwrap();
        assert!(file_b.borrow().is_file());
        assert_eq!(file_b.borrow().folder_size(), 14848514);
    }
}
