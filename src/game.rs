use crate::snake::Snake;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::style::{Print, ResetColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize};
use crossterm::ExecutableCommand;
use rand::Rng;
use rust_snake::{Command, Direction, Point};
use std::io::Stdout;
use std::time::{Duration, Instant};

pub struct Game {
    stdout: Stdout,
    width: u16,
    height: u16,
    snake: Snake,
    food: Vec<Point>,
}

impl Game {
    pub fn new(stdout: Stdout) -> Game {
        let (x, y) = size().unwrap();
        Game {
            stdout,
            width: x,
            height: y,
            snake: Snake::new(x, y),
            food: vec![],
        }
    }

    fn new_food(&mut self) {
        loop {
            let x = rand::thread_rng().gen_range(1..self.width - 1);
            let y = rand::thread_rng().gen_range(1..self.height - 1);
            let p = Point::new(x, y);
            if !self.snake.body.contains(&p) && !self.food.contains(&p) {
                self.food.push(p);
                break;
            }
        }
    }

    fn hit_wall(&self) -> bool {
        let p = self.snake.head();
        if p.x < 1 || p.x >= self.width - 1 || p.y < 1 || p.y >= self.height - 1 {
            return true;
        }
        return false;
    }

    fn hit_food(&self) -> Option<usize> {
        self.food.iter().position(|&e| e == self.snake.head())
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

    fn stop_ui(&mut self) {
        self.stdout
            .execute(SetSize(self.width, self.height))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap()
            .execute(Show)
            .unwrap()
            .execute(ResetColor)
            .unwrap();
        disable_raw_mode().unwrap();
    }

    fn draw_background(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.stdout
                    .execute(MoveTo(x, y))
                    .unwrap()
                    .execute(Print(" "))
                    .unwrap();
            }
        }
    }

    fn draw_border(&mut self) {
        for y in 0..self.height {
            if y == 0 || y == self.height - 1 {
                for x in 0..self.width {
                    self.stdout
                        .execute(MoveTo(x, y))
                        .unwrap()
                        .execute(Print("#"))
                        .unwrap();
                }
                continue;
            }
            self.stdout
                .execute(MoveTo(0, y))
                .unwrap()
                .execute(Print("#"))
                .unwrap()
                .execute(MoveTo(self.width, y))
                .unwrap()
                .execute(Print("#"))
                .unwrap();
        }
    }

    fn draw_snake(&mut self) {
        let b = self.snake.body.clone();
        for p in b.iter() {
            self.stdout
                .execute(MoveTo(p.x, p.y))
                .unwrap()
                .execute(Print("@"))
                .unwrap();
        }
    }

    fn draw_food(&mut self) {
        for f in self.food.clone() {
            self.stdout
                .execute(MoveTo(f.x, f.y))
                .unwrap()
                .execute(Print("*"))
                .unwrap();
        }
    }

    fn render(&mut self) {
        self.draw_background();
        self.draw_border();
        self.draw_food();
        self.draw_snake();
    }

    fn await_event(&self, d: Duration) -> Option<KeyEvent> {
        if poll(d).ok()? {
            let e = read().ok()?;
            if let Event::Key(k) = e {
                return Some(k);
            }
        }
        None
    }

    fn command(&self, d: Duration) -> Option<Command> {
        let k = self.await_event(d)?;
        match k.code {
            KeyCode::Char('q') => Some(Command::Quit),
            KeyCode::Up => Some(Command::Turn(Direction::Up)),
            KeyCode::Down => Some(Command::Turn(Direction::Down)),
            KeyCode::Left => Some(Command::Turn(Direction::Left)),
            KeyCode::Right => Some(Command::Turn(Direction::Right)),
            _ => None,
        }
    }

    pub fn run(&mut self) {
        for _ in 0..6 {
            self.new_food();
        }

        self.start_ui();
        self.render();
        let mut game_over = false;

        while !game_over {
            let interval = Duration::from_millis(200);
            let now = Instant::now();
            let facing = self.snake.facing;

            while now.elapsed() < interval {
                if let Some(command) = self.command(interval - now.elapsed()) {
                    match command {
                        Command::Quit => {
                            game_over = true;
                            break;
                        }
                        Command::Turn(direction) => {
                            if direction != facing.opposite() {
                                self.snake.facing = direction;
                            }
                        }
                    }
                }
            }

            self.snake.advance();

            if self.hit_wall() || self.snake.hit_self() {
                println!("you died");
                game_over = true;
                continue;
            }

            if let Some(i) = self.hit_food() {
                self.snake.eating = true;
                self.food.remove(i);
                self.new_food();
            }

            self.render();
        }
        self.stop_ui();
    }
}
