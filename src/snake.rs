use rust_snake::{Point, Direction};

pub struct Snake {
    // todo: vector
    pub body: Vec<Point>,
    pub facing: Direction,
    // todo: eating
    // eating: bool,
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
        }
    }

    pub fn head(&self) -> Point {
        self.body.first().unwrap().clone()
    }

    pub fn advance(&mut self) {
        let p = self.head().transform(self.facing);
        self.body.insert(0, p);
        self.body.remove(self.body.len() - 1);
    }
}

