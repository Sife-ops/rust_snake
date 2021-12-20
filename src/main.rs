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
            body: Point::new(3, 3),
            facing: Direction::Right,
        }
    }

    fn head(&self) -> Point {
        self.body
    }

    fn advance(&mut self) {
        match self.facing {
            x @ Direction::Up => self.body = self.body.transform(x),
            x @ Direction::Down => self.body = self.body.transform(x),
            x @ Direction::Left => self.body = self.body.transform(x),
            x @ Direction::Right => self.body = self.body.transform(x),
        }
    }
}

use crossterm::terminal::size;
use rand::Rng;
use std::io::{Stdout, stdout};
use std::time::{Duration, Instant};

struct Game {
    stdout: Stdout,
    term_size: (u16, u16),
    width: u16,
    height: u16,
    snake: Snake,
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

    fn hit_wall(&self) -> bool {
        let p = self.snake.head();
        if p.x < 1 || p.x > self.width - 1 || p.y < 1 || p.y > self.height - 1 {
            return true;
        }
        return false;
    }

    fn run(&mut self) {
        let mut game_over = false;
        while !game_over {
            let interval = Duration::from_millis(1000);
            let now = Instant::now();
            while now.elapsed() < interval {
                //;
            }
            if self.hit_wall() {
                game_over = true;
            }
            println!("{:?}", self.snake.body);
        }
    }
}

fn main() {
    Game::new(stdout(), 10, 10).run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_transform_up() {
        let p = Point::new(3, 3);
        let p = p.transform(Direction::Up);
        assert_eq!(p, Point::new(3, 2));
    }

    #[test]
    fn test_point_transform_down() {
        let p = Point::new(3, 3);
        let p = p.transform(Direction::Down);
        assert_eq!(p, Point::new(3, 4));
    }

    #[test]
    fn test_point_transform_left() {
        let p = Point::new(3, 3);
        let p = p.transform(Direction::Left);
        assert_eq!(p, Point::new(2, 3));
    }

    #[test]
    fn test_point_transform_right() {
        let p = Point::new(3, 3);
        let p = p.transform(Direction::Right);
        assert_eq!(p, Point::new(4, 3));
    }

    #[test]
    fn test_snake_advance_up() {
        let mut s = Snake::new();
        let mut p = s.body.clone();
        p.y = p.y - 1;
        s.facing = Direction::Up;
        s.advance();
        assert_eq!(s.body, p);
    }

    #[test]
    fn test_snake_advance_down() {
        let mut s = Snake::new();
        let mut p = s.body.clone();
        p.y = p.y + 1;
        s.facing = Direction::Down;
        s.advance();
        assert_eq!(s.body, p);
    }

    #[test]
    fn test_snake_advance_left() {
        let mut s = Snake::new();
        let mut p = s.body.clone();
        p.x = p.x - 1;
        s.facing = Direction::Left;
        s.advance();
        assert_eq!(s.body, p);
    }

    #[test]
    fn test_snake_advance_right() {
        let mut s = Snake::new();
        let mut p = s.body.clone();
        p.x = p.x + 1;
        s.facing = Direction::Right;
        s.advance();
        assert_eq!(s.body, p);
    }

}
