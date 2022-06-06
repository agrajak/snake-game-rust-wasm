pub type Position = (usize, usize);

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

use crate::snake::Snake;

impl Direction {
    pub fn getTuple(&self) -> (i8, i8) {
        match *self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
    pub fn add_position(&self, (x, y): Position) -> Position {
        let (dx, dy) = self.getTuple();
        ((dx + x as i8) as usize, (dy + x as i8) as usize)
    }
}

pub enum SnakeMoveResult {
    Done,
    Dead,
    Grow(Position),
}

// https://stackoverflow.com/a/29437510
#[derive(Copy, Clone)]
enum GridInfo {
    Empty,
    Apple,
    SnakeHead,
    SnakeBody,
}

pub struct SnakeGame {
    width: usize,
    height: usize,
    snake: Snake,
    apple: Option<Position>,
    speed: f32,
}

impl SnakeGame {
    fn finish_game(&mut self) {
        self.speed = 0.0;
    }
    pub fn tick(&mut self) {
        let result = self.move_snake();
        match result {
            SnakeMoveResult::Done => self.snake.do_move(),
            SnakeMoveResult::Grow(new_position) => self.snake.grow(new_position),
            SnakeMoveResult::Dead => self.finish_game(),
        }
    }
    fn move_snake(&self) -> SnakeMoveResult {
        let new_position = self.snake.estimate_new_head_position();
        let (x, y) = new_position;
        if !((0..self.width).contains(&x) && (0..self.height).contains(&y)) {
            // 벽에 충돌함 ㅋ
            return SnakeMoveResult::Dead;
        }

        if self.snake.has_body_in(new_position) {
            return SnakeMoveResult::Dead;
        }

        match self.apple {
            None => SnakeMoveResult::Done,
            Some(apple_position) => {
                if new_position.eq(&apple_position) {
                    return SnakeMoveResult::Grow(new_position);
                }
                SnakeMoveResult::Done
            }
        }
    }

    pub fn print(&self) {
        let mut grid = vec![vec![GridInfo::Empty as GridInfo; self.width]; self.height];

        for (i, (x, y)) in self.snake.get_body_iter().enumerate() {
            grid[*x][*y] = match i {
                0 => GridInfo::SnakeHead,
                _ => GridInfo::SnakeBody,
            }
        }

        match self.apple {
            None => (),
            Some((x, y)) => {
                grid[x][y] = GridInfo::Apple;
            }
        }

        for i in 0..self.height {
            for j in 0..self.width {
                print!(
                    "{}",
                    match grid[i][j] {
                        GridInfo::Empty => " .",
                        GridInfo::Apple => "🍎",
                        GridInfo::SnakeHead => "🐍",
                        GridInfo::SnakeBody => "🟩",
                    }
                );
            }
            println!();
        }
        println!()
    }
    pub fn start_game(size: usize) -> SnakeGame {
        SnakeGame {
            width: (size),
            height: (size),
            snake: crate::snake::Snake::make_snake((size/2, size/2), Direction::Up),
            apple: (None),
            speed: (1.0),
        }
    }
}
