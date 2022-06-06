
use std::collections::{vec_deque::Iter, VecDeque};

use crate::snake_game::{Position, Direction};

pub struct Snake {
    body: VecDeque<Position>,
    direction: Direction,
}

impl Snake {
    pub fn make_snake(start_position: Position, start_direction: Direction) -> Snake {
        Snake { body: (VecDeque::<Position>::from([start_position])), direction: (start_direction) }
    }
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    pub fn get_head(&self) -> Position {
        self.body[0]
    }
    // 새로운 머리 위치를 예측
    pub fn estimate_new_head_position(&self) -> Position {
        let old_head = self.get_head();
        self.direction.add_position(old_head)
    }
    pub fn has_body_in(&self, position: Position) -> bool {
        self.body.contains(&position)
    }
    pub fn get_body_iter(&self) -> Iter<Position> {
        self.body.iter()
    }
    pub fn do_move(&mut self) {
        let new_head = self.estimate_new_head_position();
        self.body.push_front(new_head);
        self.body.pop_back();
    }
    pub fn grow(&mut self, new_position: Position) {
        self.body.push_front(new_position);
    }
}
