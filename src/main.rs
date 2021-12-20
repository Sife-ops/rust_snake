use crossterm::event::{
    poll,
    read,
    //;
    Event,
    KeyCode,
    KeyEvent,
};

use crossterm::terminal::{
    disable_raw_mode,
    enable_raw_mode,
    size,
    //;
    Clear,
    ClearType,
    SetSize,
};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::ExecutableCommand;
use rand::Rng;
use std::io::{stdout, Stdout};
use std::time::{Duration, Instant};

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
    body: Vec<Point>,
    facing: Direction,
    // todo: eating
    // eating: bool,
}

impl Snake {
    fn new() -> Self {
        Snake {
            body: vec![Point::new(3, 3), Point::new(2, 3), Point::new(1, 3)],
            facing: Direction::Right,
        }
    }

    fn head(&self) -> Point {
        self.body.first().unwrap().clone()
    }

    fn advance(&mut self) {
        let p = self.head().transform(self.facing);
        self.body.insert(0, p);
        self.body.remove(self.body.len() - 1);
    }
}

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

    fn start_ui(&mut self) {
        enable_raw_mode().unwrap();
        self.stdout
            .execute(SetSize(self.width + 3, self.height + 3))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap()
            .execute(Hide)
            .unwrap();
    }

    fn draw_background(&mut self) {
        // self.stdout.execute(ResetColor).unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                self.stdout
                    .execute(MoveTo(x, y))
                    .unwrap()
                    .execute(Print("0"))
                    .unwrap();
            }
        }
    }

    fn draw_snake(&mut self) {
        self.stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        let b = self.snake.body.clone();
        println!("{:?}", b);
        for (i, p) in b.iter().enumerate() {
            self.stdout
                .execute(MoveTo(p.x, p.y))
                .unwrap()
                .execute(Print("8"))
                .unwrap();
        }
    }

    fn render(&mut self) {
        self.draw_background();
        self.draw_snake();
    }

    fn run(&mut self) {
        self.start_ui();
        self.render();
        let mut game_over = false;
        while !game_over {
            let interval = Duration::from_millis(1000);
            let now = Instant::now();

            // println!("{:?}", self.snake.body);

            while now.elapsed() < interval {
                //;
            }

            if self.hit_wall() {
                game_over = true;
                println!("you died");
            } else {
                self.snake.advance();
                self.render();
            }
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

    // #[test]
    // fn test_snake_advance_up() {
    //     let mut s = Snake::new();
    //     let mut p = s.body.clone();
    //     p.y = p.y - 1;
    //     s.facing = Direction::Up;
    //     s.advance();
    //     assert_eq!(s.body, p);
    // }

    // #[test]
    // fn test_snake_advance_down() {
    //     let mut s = Snake::new();
    //     let mut p = s.body.clone();
    //     p.y = p.y + 1;
    //     s.facing = Direction::Down;
    //     s.advance();
    //     assert_eq!(s.body, p);
    // }

    // #[test]
    // fn test_snake_advance_left() {
    //     let mut s = Snake::new();
    //     let mut p = s.body.clone();
    //     p.x = p.x - 1;
    //     s.facing = Direction::Left;
    //     s.advance();
    //     assert_eq!(s.body, p);
    // }

    // #[test]
    // fn test_snake_advance_right() {
    //     let mut s = Snake::new();
    //     let mut p = s.body.clone();
    //     p.x = p.x + 1;
    //     s.facing = Direction::Right;
    //     s.advance();
    //     assert_eq!(s.body, p);
    // }
}
