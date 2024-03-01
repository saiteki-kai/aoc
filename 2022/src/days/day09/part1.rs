// https://adventofcode.com/2022/9
//
// Day 9: Rope Bridge - Part 1

use std::fmt;

pub fn solve() {
    let input: &str = include_str!("../../../inputs/day09");

    let moves = parse_moves(input);
    let count = count_tail_positions(moves);

    println!("{}", count);
}

fn parse_moves(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .map(|line| {
            let (dir, step) = line.trim().split_once(' ').unwrap();
            let direction = match dir {
                "U" => Direction::U,
                "L" => Direction::L,
                "R" => Direction::R,
                _ => Direction::D,
            };

            (direction, step.parse::<i32>().unwrap())
        })
        .collect()
}

fn count_tail_positions(moves: Vec<(Direction, i32)>) -> usize {
    let start = Point::new(0, 0);
    let mut visited: Vec<Point> = vec![start];

    let mut rope = Rope::new(start);

    for m in moves {
        println!("{:?}, {}", m.0, m.1);

        for _ in 0..(m.1) {
            rope.move_to(&m.0);

            if !visited.contains(&rope.tail) {
                visited.push(rope.tail)
            }
        }
    }

    visited.len()
}

#[derive(Clone, Debug)]
pub enum Direction {
    U,
    R,
    L,
    D,
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::R => self.x += 1,
            Direction::L => self.x -= 1,
            Direction::D => self.y += 1,
            Direction::U => self.y -= 1,
        }
    }

    fn move_diagonal_to(&mut self, direction_x: &Direction, direction_y: &Direction) {
        self.move_to(direction_x);
        self.move_to(direction_y);
    }

    fn gradient(point1: &Point, point2: &Point) -> Point {
        Point {
            x: point2.x - point1.x,
            y: point2.y - point1.y,
        }
    }

    fn distance_to(&self, point: &Point) -> i32 {
        (self.x - point.x).abs().max((self.y - point.y).abs())
    }

    fn is_adjacent_to(&self, point: &Point) -> bool {
        self.distance_to(point) <= 1
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Rope {
    head: Point,
    tail: Point,
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "head: {}, tail: {}", self.head, self.tail)
    }
}

impl Rope {
    fn new(start: Point) -> Rope {
        Rope {
            head: start,
            tail: start,
        }
    }

    fn move_toward_to_head(&mut self) {
        let p = Point::gradient(&self.tail, &self.head);

        let dir_x = if p.x > 0 { Direction::R } else { Direction::L };
        let dir_y = if p.y > 0 { Direction::D } else { Direction::U };

        self.tail.move_diagonal_to(&dir_x, &dir_y);
    }

    fn move_head(&mut self, direction: &Direction) {
        self.head.move_to(direction);
    }

    fn move_tail(&mut self, direction: &Direction) {
        // two steps directly up, down, left, or right from the tail
        if self.tail.distance_to(&self.head) == 2 && self.is_same_row_or_col(direction) {
            self.tail.move_to(direction)
        }

        // the head and tail aren't touching and aren't in the same row or column
        if !self.is_same_row_or_col(direction) && !self.is_touching() {
            self.move_toward_to_head()
        }
    }

    fn move_to(&mut self, direction: &Direction) {
        self.move_head(direction);
        self.move_tail(direction);

        println!("{}", self);
    }

    fn is_same_row_or_col(&self, direction: &Direction) -> bool {
        match direction {
            Direction::L | Direction::R => self.head.y == self.tail.y,
            Direction::U | Direction::D => self.head.x == self.tail.x,
        }
    }

    fn is_adjacent(&self) -> bool {
        self.head.is_adjacent_to(&self.tail)
    }

    fn is_touching(&self) -> bool {
        self.head == self.tail || self.is_adjacent()
    }
}

#[cfg(test)]
mod tests {
    use super::{count_tail_positions, Direction, Point, Rope};

    use test_case::test_case;

    #[test_case(Point::new(-3, 1), Point::new(3, 4), false)]
    #[test_case(Point::new(-3, 4), Point::new(3, -4), false)]
    #[test_case(Point::new(2, 2), Point::new(2, 2), true)]
    #[test_case(Point::new(-3, -3), Point::new(-3, -3), true)]
    #[test_case(Point::new(0, 0), Point::new(1, 1), true; "top right")]
    #[test_case(Point::new(0, 0), Point::new(-1, -1), true; "bottom left")]
    #[test_case(Point::new(0, 0), Point::new(-1, 0), true; "left center")]
    #[test_case(Point::new(0, 0), Point::new(1, 0), true; "right center")]
    fn test_touching(head: Point, tail: Point, value: bool) {
        let rope = Rope { head, tail };
        assert_eq!(rope.is_touching(), value);
    }

    #[test]
    fn test_moves() {
        let moves = [
            (Direction::R, 4),
            (Direction::U, 4),
            (Direction::L, 3),
            (Direction::D, 1),
            (Direction::R, 4),
            (Direction::D, 1),
            (Direction::L, 5),
            (Direction::R, 2),
        ]
        .to_vec();

        let count = count_tail_positions(moves);
        assert_eq!(count, 13);
    }
}
