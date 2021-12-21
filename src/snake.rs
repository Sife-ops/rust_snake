use rust_snake::{Direction, Point};

pub struct Snake {
    pub body: Vec<Point>,
    pub facing: Direction,
    pub eating: bool,
}

impl Snake {
    pub fn new(w: u16, h: u16) -> Snake {
        let x = w / 2;
        let y = h / 2;
        Snake {
            body: vec![
                //;
                Point::new(x, y),
                Point::new(x - 1, y),
                Point::new(x - 2, y),
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
