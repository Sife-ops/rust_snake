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
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    fn transform(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new(self.x, self.y - 1),
            Direction::Down => Self::new(self.x, self.y + 1),
            Direction::Left => Self::new(self.x - 1, self.y),
            Direction::Right => Self::new(self.x + 1, self.y),
        }
    }
}

struct Snake {
    // todo: vector
    body: Point,
    facing: Direction,
    // todo: eating
    // eating: bool,
}

impl Snake {
    // todo: random facing direction
    // fn new(start: Point, length: u16, direction: Direction) -> {
    // }

    fn new() -> Self {
        Snake {
            body: Point::new(3,3),
            facing: Direction::Right,
        }
    }
}

use std::io::Stdout;
use std::time::{Duration, Instant};
use crossterm::terminal::size;
use rand::Rng;

struct Game {
    stdout: Stdout,
    term_size: (u16,u16),
    width: u16,
    height: u16,
    snake: Snake
}

impl Game {
    fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        Self {
            stdout,
            term_size: size().unwrap(),
            width,
            height,
            snake: Snake::new(),
        }
    }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_transform_up() {
        let p = Point::new(3,3);
        let p = p.transform(Direction::Up);
        assert_eq!(p, Point::new(3,2));
    }

    #[test]
    fn test_point_transform_down() {
        let p = Point::new(3,3);
        let p = p.transform(Direction::Down);
        assert_eq!(p, Point::new(3,4));
    }

    #[test]
    fn test_point_transform_left() {
        let p = Point::new(3,3);
        let p = p.transform(Direction::Left);
        assert_eq!(p, Point::new(2,3));
    }

    #[test]
    fn test_point_transform_right() {
        let p = Point::new(3,3);
        let p = p.transform(Direction::Right);
        assert_eq!(p, Point::new(4,3));
    }

}