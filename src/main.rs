mod game;
mod snake;

use game::Game;
use std::io::stdout;

fn main() {
    Game::new(stdout(), 10, 10).run();
}
