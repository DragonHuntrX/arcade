use rand::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Playing,
    Over,
}

pub struct Game {
    pub world: Vec<i8>,
    pub snake: Vec<usize>,
    pub food: Vec<usize>,
    pub dir: Direction,
    pub new_dir: Option<Direction>,
    pub rows: i32,
    pub cols: i32,
    pub state: GameState,
    pub score: i64,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            world: vec![0; 30 * 20],
            snake: vec![(30 * 10) + 15],
            dir: Direction::Right,
            new_dir: None,
            food: vec![],
            rows: 20,
            cols: 30,
            state: GameState::Playing,
            score: 0,
        };
        game.spawn_food();
        game.update_world();
        game
    }

    pub fn step(&mut self) -> i64 {
        let mut score = 0;
        if let Some(new_dir) = self.new_dir {
            if (new_dir == Direction::Down && self.dir != Direction::Up)
                || (new_dir == Direction::Right && self.dir != Direction::Left)
                || (new_dir == Direction::Up && self.dir != Direction::Down)
                || (new_dir == Direction::Left && self.dir != Direction::Right)
                || self.snake.len() == 1
            {
                self.dir = new_dir;
            }
        }

        let mut head = self.snake[0] as i32;
        match self.dir {
            Direction::Up => head -= self.cols,
            Direction::Down => head += self.cols,
            Direction::Left => head -= 1,
            Direction::Right => head += 1,
        }
        if self.dir == Direction::Right && head % self.cols == 0
            || self.dir == Direction::Left && head % self.cols == self.cols - 1
            || head < 0
            || head >= self.cols * self.rows
        {
            self.state = GameState::Over;
        }
        if self.snake.contains(&(head as usize)) {
            self.state = GameState::Over
        }

        if self.state != GameState::Over {
            let mut grow = false;
            for i in 0..self.food.len() {
                if self.food[i] == head as usize {
                    self.food.remove(i);
                    self.spawn_food();
                    grow = true;
                    score += 1;
                }
            }

            let tail = self.snake[self.snake.len() - 1];
            for i in (1..self.snake.len()).rev() {
                self.snake[i] = self.snake[i - 1];
            }
            self.snake[0] = head as usize;
            if grow {
                self.snake.push(tail);
            }
        }
        self.update_world();
        score
    }

    pub fn set_heading(&mut self, dir: Direction) {
        self.new_dir = Some(dir);
    }

    pub fn spawn_food(&mut self) {
        let mut index: usize = thread_rng().gen_range(0..(self.rows * self.cols) as usize);

        while self.food.contains(&index) || self.snake.contains(&index) {
            index = thread_rng().gen_range(0..(self.rows * self.cols) as usize);
            println!("{}", index);
        }
        self.food.push(index as usize);
    }

    fn update_world(&mut self) {
        let mut new_world = vec![0; (self.rows * self.cols) as usize];
        for &snake_part in &self.snake {
            new_world[snake_part] = 1;
        }
        for &food in &self.food {
            new_world[food] = 2;
        }
        self.world = new_world;
    }
}
