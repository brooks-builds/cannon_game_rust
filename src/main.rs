extern crate cannon_game;
use bbggez::run::run_dim;
use cannon_game::Game;

fn main() {
    let mut game = Game::new();

    run_dim(&mut game, 1500.0, 250.0, "Cannon Game", "Brookzerker");
}
