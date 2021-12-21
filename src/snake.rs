use rust_snake::{Point, Direction};

pub struct Snake {
    pub body: Vec<Point>,
    pub facing: Direction,
    pub eating: bool,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            body: vec![
                //;
                Point::new(3, 3),
                Point::new(2, 3),
                Point::new(1, 3),
            ],
            facing: Direction::Right,
            eating: false,
        }
    }

    pub fn head(&self) -> Point {
        self.body.first().unwrap().clone()
    }

    pub fn hit_self(&self) -> bool {
        let b = &self.body[1..];
        b.contains(&self.head())
    }

    pub fn advance(&mut self) {
        let p = self.head().transform(self.facing);
        self.body.insert(0, p);
        if self.eating {
            self.eating = false;
            return;
        }
        self.body.remove(self.body.len() - 1);
    }
}

