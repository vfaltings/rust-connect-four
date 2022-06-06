pub mod connectfour;
use crate::connectfour::*;

fn main() {
    let mut game = Game::new();
    game.play();
}
