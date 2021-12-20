#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

enum Command {
    Quit,
    Turn(Direction),
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn transform(&self, direction: Direction, times: u32) -> Self {
        let times = times as i32;
        let transformation = match direction {
            Direction::Up => (0, -times),
            Direction::Down => (0, times),
            Direction::Left => (-times, 0),
            Direction::Right => (times, 0),
        };
        Self::new(
            Self::transform_value(self.x, transformation.0),
            Self::transform_value(self.y, transformation.1),
        )
    }

    fn transform_value(value: u32, by: i32) -> u32 {
        if by.is_negative() && by.abs() as u32 > value {
            panic!("negative position");
        } else {
            return (value as i32 + by) as u32;
        }
    }
}

struct Snake {
    body: Vec<Point>,
    direction: Direction,
    eating: bool,
}

impl Snake {
    // todo
    // fn new(start: Point, length: u32, direction: Direction) -> {
    // }

    fn new() -> Self {
        Snake {
            body: vec![Point::new(3, 3), Point::new(2, 3)],
            direction: Direction::Right,
            eating: false,
        }
    }

    fn head(&self) -> Point {
        self.body.first().unwrap().clone()
    }

    fn body(&self) -> Vec<Point> {
        self.body.clone()
    }

    fn direction(&self) -> Direction {
        self.direction.clone()
    }

    fn contains(&self, point: &Point) -> bool {
        self.body.contains(point)
    }

    fn advance(&mut self) {
        self.body.insert(
            //;
            0,
            self.body.first().unwrap().transform(self.direction, 1),
        );
        if self.eating {
            self.eating = false;
        } else {
            self.body.remove(self.body.len() - 1);
        }
    }

    fn face(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn eat(&mut self) {
        self.eating = true;
    }
}

use std::io::Stdout;
use std::time::{Duration, Instant};
use crossterm::terminal::size;
use rand::Rng;

const MAX_INTERVAL: u32 = 700;
const MIN_INTERVAL: u32 = 200;
const MAX_SPEED: u32 = 20;

fn main() {
    println!("Hello, world!");
}
