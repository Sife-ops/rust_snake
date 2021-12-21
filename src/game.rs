use crate::snake::Snake;
use crossterm::ExecutableCommand;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::style::{Print, ResetColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize};
use rand::Rng;
use rust_snake::{Command, Direction};
use std::io::Stdout;
use std::time::{Duration, Instant};

pub struct Game {
    pub stdout: Stdout,
    pub term_size: (u16, u16),
    pub width: u16,
    pub height: u16,
    pub snake: Snake,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        Self {
            stdout,
            term_size: size().unwrap(),
            width,
            height,
            snake: Snake::new(),
        }
    }

    pub fn hit_wall(&self) -> bool {
        let p = self.snake.head();
        if p.x < 1 || p.x >= self.width - 1 || p.y < 1 || p.y >= self.height - 1 {
            return true;
        }
        return false;
    }

    pub fn start_ui(&mut self) {
        enable_raw_mode().unwrap();
        self.stdout
            .execute(SetSize(self.width + 3, self.height + 3))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap()
            .execute(Hide)
            .unwrap();
    }

    pub fn stop_ui(&mut self) {
        let (cols, rows) = self.term_size;
        self.stdout
            .execute(SetSize(cols, rows))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap()
            .execute(Show)
            .unwrap()
            .execute(ResetColor)
            .unwrap();
        disable_raw_mode().unwrap();
    }

    pub fn draw_background(&mut self) {
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

    pub fn draw_snake(&mut self) {
        // self.stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        let b = self.snake.body.clone();
        // for (i, p) in b.iter().enumerate() {
        for p in b.iter() {
            self.stdout
                .execute(MoveTo(p.x, p.y))
                .unwrap()
                .execute(Print("8"))
                .unwrap();
        }
    }

    pub fn render(&mut self) {
        self.draw_background();
        self.draw_snake();
    }

    pub fn await_event(&self, d: Duration) -> Option<KeyEvent> {
        if poll(d).ok()? {
            let e = read().ok()?;
            if let Event::Key(k) = e {
                return Some(k);
            }
        }
        None
    }

    pub fn command(&self, d: Duration) -> Option<Command> {
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
        self.start_ui();
        self.render();
        let mut game_over = false;
        while !game_over {
            let interval = Duration::from_millis(1000);
            let now = Instant::now();

            while now.elapsed() < interval {
                if let Some(c) = self.command(interval - now.elapsed()) {
                    match c {
                        Command::Quit => {
                            game_over = true;
                            break;
                        }
                        Command::Turn(d) => {
                            if d != self.snake.facing.opposite() {
                                self.snake.facing = d;
                            }
                        }
                    }
                }
            }

            if self.hit_wall() {
                println!("you died");
                game_over = true;
            } else {
                self.snake.advance();
                self.render();
            }
        }
        self.stop_ui();
    }
}