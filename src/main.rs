mod snake_game;
mod snake;

use crate::snake_game::SnakeGame;

fn main() {
    let mut game = SnakeGame::start_game(10);
    game.print();
    game.tick();
    game.print();
    game.tick();
    game.print();
    game.tick();
}
