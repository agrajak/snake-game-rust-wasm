pub mod snake_game;
pub mod snake;

use wasm_bindgen::prelude::*;
use crate::snake_game::{SnakeGame};

#[wasm_bindgen]
pub fn start_game(size: usize) -> JsValue {
    JsValue::from_serde(&SnakeGame::start_game(size)).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut game = SnakeGame::start_game(10);
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
}