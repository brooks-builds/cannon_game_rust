extern crate cannon_game;
use bbggez::run::run_dim;
use cannon_game::Game;

fn main() {
    let game = Game::new();

    run_dim(
        &mut game.unwrap(),
        1500.0,
        250.0,
        "Cannon Game",
        "Brookzerker",
    );
}
