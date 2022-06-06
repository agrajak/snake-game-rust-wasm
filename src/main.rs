mod snake_game;
mod snake;

use crate::snake_game::{SnakeGame, Direction};

fn main() {
    let mut game = SnakeGame::start_game(10);
    game.set_apple_position((3,5));
    game.print();

    game.tick();
    game.print();

    game.tick();
    game.print();

    game.tick();
    game.print();

    game.tick();
    game.print();

    game.snake.set_direction(Direction::Right);

    game.tick();
    game.print();

    game.tick();
    game.print();

    game.tick();
    game.print();


}
