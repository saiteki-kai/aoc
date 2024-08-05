// https://adventofcode.com/2022/day/9
//
// Day 9: Rope Bridge - Part 2

use std::fmt;

use super::shared::{parse_moves, Direction, Point};

pub fn solve() {
    let input: &str = include_str!("../../../inputs/day09");

    let moves = parse_moves(input);
    let count = count_tail_positions(moves);

    println!("{}", count);
}

fn count_tail_positions(moves: Vec<(Direction, i32)>) -> usize {
    let start = Point::new(0, 0);
    let mut visited: Vec<Point> = vec![start];

    let mut rope = Rope::new(start);

    for m in moves {
        println!("{:?}, {}", m.0, m.1);

        for _ in 0..(m.1) {
            rope.move_to(&m.0);

            let tail = rope.knots.last().unwrap();

            if !visited.contains(tail) {
                visited.push(*tail)
            }
        }
    }

    visited.len()
}

#[derive(Clone)]
struct Rope {
    knots: Vec<Point>,
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let body = self
            .knots
            .iter()
            .map(|k| format!("({}, {})", k.x, k.y))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{}", body)
    }
}

impl Rope {
    fn new(start: Point) -> Rope {
        Rope {
            knots: [start; 10].to_vec(),
        }
    }

    fn move_to(&mut self, direction: &Direction) {
        Rope::move_head(&mut self.knots[0], direction);

        for i in 1..self.knots.len() {
            let (head, tail) = self.knots.split_at_mut(i);
            Rope::move_tail(&head[i - 1], &mut tail[0], direction);
        }
        println!("{}", self);
    }

    fn is_same_row_or_col(head: &Point, tail: &Point) -> bool {
        head.y == tail.y || head.x == tail.x
    }

    fn is_adjacent(head: &Point, tail: &Point) -> bool {
        head.is_adjacent_to(tail)
    }

    fn is_touching(head: &Point, tail: &Point) -> bool {
        head == tail || Rope::is_adjacent(head, tail)
    }

    fn move_toward_to_head(tail: &mut Point, head: &Point) {
        let p = Point::gradient(tail, head);

        let dir_x = if p.x > 0 { Direction::R } else { Direction::L };
        let dir_y = if p.y > 0 { Direction::D } else { Direction::U };

        tail.move_diagonal_to(&dir_x, &dir_y);
    }

    fn move_head(head: &mut Point, direction: &Direction) {
        head.move_to(direction);
    }

    fn move_tail(head: &Point, tail: &mut Point, direction: &Direction) {
        // two steps directly up, down, left, or right from the tail
        if tail.distance_to(head) == 2 && Rope::is_same_row_or_col(head, tail) {
            tail.move_to(direction)
        }

        // the head and tail aren't touching and aren't in the same row or column
        if !Rope::is_same_row_or_col(head, tail) && !Rope::is_touching(head, tail) {
            Rope::move_toward_to_head(tail, head)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{count_tail_positions, Direction};
    use test_case::test_case;

    #[test_case([
        (Direction::R, 4),
        (Direction::U, 4),
        (Direction::L, 3),
        (Direction::D, 1),
        (Direction::R, 4),
        (Direction::D, 1),
        (Direction::L, 5),
        (Direction::R, 2),
    ].to_vec(), 1)]
    #[test_case([
        (Direction::R, 5),
        (Direction::U, 8),
        (Direction::L, 8),
        (Direction::D, 3),
        (Direction::R, 17),
        (Direction::D, 10),
        (Direction::L, 25),
        (Direction::U, 20),
    ].to_vec(), 36)]
    fn test_moves(moves: Vec<(Direction, i32)>, value: usize) {
        let count = count_tail_positions(moves);
        assert_eq!(count, value);
    }
}
