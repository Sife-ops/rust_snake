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
    term_size: (u16, u16),
    width: u16,
    height: u16,
    snake: Snake,
    food: Option<Point>,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Game {
        Game {
            stdout,
            term_size: size().unwrap(),
            width,
            height,
            snake: Snake::new(),
            food: None,
        }
    }

    fn spawn_food(&mut self) {
        loop {
            let x = rand::thread_rng().gen_range(1..self.width - 1);
            let y = rand::thread_rng().gen_range(1..self.height - 1);
            let p = Point::new(x, y);
            if self.snake.body.contains(&p) {
                continue;
            }
            self.food = Some(Point::new(x, y));
            break;
        }
    }

    fn hit_wall(&self) -> bool {
        let p = self.snake.head();
        if p.x < 1 || p.x >= self.width - 1 || p.y < 1 || p.y >= self.height - 1 {
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

    fn stop_ui(&mut self) {
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

    fn draw_background(&mut self) {
        // self.stdout.execute(ResetColor).unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                self.stdout
                    .execute(MoveTo(x, y))
                    .unwrap()
                    .execute(Print("."))
                    .unwrap();
            }
        }
    }

    fn draw_snake(&mut self) {
        // self.stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        let b = self.snake.body.clone();
        // for (i, p) in b.iter().enumerate() {
        for p in b.iter() {
            self.stdout
                .execute(MoveTo(p.x, p.y))
                .unwrap()
                .execute(Print("@"))
                .unwrap();
        }
    }

    fn draw_food(&mut self) {
        let p = self.food.clone().unwrap();
        self.stdout
            .execute(MoveTo(p.x, p.y))
            .unwrap()
            .execute(Print("*"))
            .unwrap();
    }

    fn render(&mut self) {
        self.draw_background();
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
        self.spawn_food();
        self.start_ui();
        self.render();
        let mut game_over = false;
        while !game_over {
            let interval = Duration::from_millis(1000);
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

            if self.hit_wall() || self.snake.hit_self() {
                println!("you died");
                game_over = true;
                continue;
            } 

            if self.snake.head() == self.food.unwrap() {
                self.snake.eating = true;
                self.spawn_food();
            }

            self.snake.advance();
            self.render();
        }
        self.stop_ui();
    }
}
