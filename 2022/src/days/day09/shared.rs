use std::fmt;

#[derive(Clone, Debug)]
pub enum Direction {
    U,
    R,
    L,
    D,
}

pub fn parse_moves(input: &str) -> Vec<(Direction, i32)> {
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::R => self.x += 1,
            Direction::L => self.x -= 1,
            Direction::D => self.y += 1,
            Direction::U => self.y -= 1,
        }
    }

    pub fn move_diagonal_to(&mut self, direction_x: &Direction, direction_y: &Direction) {
        self.move_to(direction_x);
        self.move_to(direction_y);
    }

    pub fn gradient(point1: &Point, point2: &Point) -> Point {
        Point {
            x: point2.x - point1.x,
            y: point2.y - point1.y,
        }
    }

    pub fn distance_to(&self, point: &Point) -> i32 {
        (self.x - point.x).abs().max((self.y - point.y).abs())
    }

    pub fn is_adjacent_to(&self, point: &Point) -> bool {
        self.distance_to(point) <= 1
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
