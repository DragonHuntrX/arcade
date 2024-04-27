mod gamemodes;
mod utils;

use gamemodes::Gamemode;
use std::fmt::Display;

pub struct Game {
    pub board: Board,
    pub state: GameState,
    pub score: i64,
    mode: Gamemode,
}

#[derive(Clone)]
pub struct Board {
    pub state: Vec<i32>,
    pub rows: usize,
    pub cols: usize,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
        Board {
            state: vec![0; rows * cols],
            rows,
            cols,
        }
    }
    pub fn to_index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
    fn is_full(&self) -> bool {
        !self.state.iter().fold(false, |acc, &x| acc || x == 0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Start,
    Moved,
    Static,
    Over,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
impl Direction {
    const VALUES: [Self; 4] = [Self::Up, Self::Down, Self::Right, Self::Left];
}

impl Game {
    pub fn new() -> Game {
        let mut new_game = Game {
            board: Board::new(4, 4),
            state: GameState::Start,
            mode: gamemodes::NORMAL,
            score: 0,
        };

        (new_game.mode.add_block)(&mut new_game.board);
        (new_game.mode.add_block)(&mut new_game.board);

        new_game
    }
    pub fn from_r_and_c(rows: usize, cols: usize) -> Game {
        let mut new_game = Game {
            board: Board::new(rows, cols),
            state: GameState::Start,
            mode: gamemodes::NORMAL,
            score: 0,
        };

        (new_game.mode.add_block)(&mut new_game.board);
        (new_game.mode.add_block)(&mut new_game.board);

        new_game
    }

    pub fn step(&mut self, direction: Direction) {
        if self.state == GameState::Over {
            return;
        }
        let (changed, score) = (self.mode.step)(&mut self.board, direction);
        self.score += score;
        if changed {
            self.state = GameState::Moved;
            (self.mode.add_block)(&mut self.board);
            if self.board.is_full() {
                let mut stuck = true;
                for dir in Direction::VALUES {
                    let mut temp = self.board.clone();
                    stuck &= !(self.mode.step)(&mut temp, dir).0;
                }
                if stuck {
                    self.state = GameState::Over;
                }
            }
        } else {
            self.state = GameState::Static;
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let top = format!(
            "{}{}{}",
            "╔",
            "═══════╤"
                .repeat(self.board.cols)
                .split_at(self.board.cols * 24 - 3)
                .0,
            "╗"
        );
        let mid: String = format!(
            "{}{}{}",
            "╟",
            "───────┼"
                .repeat(self.board.cols)
                .split_at(self.board.cols * 24 - 3)
                .0,
            "╢"
        );
        let space: String = format!(
            "{}{}{}",
            "║",
            "\0 \0\0 \0\0 \0\0 \0\0 \0\0 \0\0 \0│"
                .repeat(self.board.cols)
                .split_at(self.board.cols * 24 - 3)
                .0,
            "║"
        );

        let bottom: String = format!(
            "{}{}{}",
            "╚",
            "═══════╧"
                .repeat(self.board.cols)
                .split_at(self.board.cols * 24 - 3)
                .0,
            "╝"
        );

        let mut output = "".to_string();

        output += format!("{}\n", top).as_str();
        let mut row = 0;
        loop {
            let mut col = 0;
            output += format!("{}\n", space).as_str();
            output += format!("║").as_str();
            loop {
                output += format!(
                    "{:^7}",
                    if self.board.state[self.board.cols * row + col] != 0 {
                        self.board.state[self.board.cols * row + col].to_string()
                    } else {
                        "".to_string()
                    }
                )
                .as_str();
                col += 1;
                if col == self.board.cols {
                    break;
                }
                output += format!("│").as_str();
            }
            output += format!("║\n").as_str();
            output += format!("{}\n", space).as_str();
            row += 1;
            if row == self.board.rows {
                break;
            }
            output += format!("{}\n", mid).as_str();
        }
        output += format!("{}\n", bottom).as_str();
        output += format!("State: {:?}", self.state).as_str();
        output += format!("Score: {}", self.score).as_str();

        write!(f, "{}", output)
    }
}

/* Graveyard
pub fn add_block(&mut self) -> bool {
        let mut seen = (0..self.board.len() as i32).collect::<Vec<i32>>();
        let mut rng = thread_rng();

        while seen.len() > 0 {
            let index = rng.gen_range(0..seen.len());
            let spot = seen[index] as usize;
            if self.board[spot] == 0 {
                self.board[spot] = [(1, 8), (2, 2)]
                    .choose_weighted(&mut rng, |z| z.1)
                    .unwrap()
                    .0;
                return true;
            }

            seen.remove(index);
        }

        false
    }

    pub fn display(&self) {
        let top: String = format!(
            "{}{}{}",
            "╔",
            "═══════╤"
                .repeat(self.columns)
                .split_at(self.columns * 24 - 3)
                .0,
            "╗"
        );
        let mid: String = format!(
            "{}{}{}",
            "╟",
            "───────┼"
                .repeat(self.columns)
                .split_at(self.columns * 24 - 3)
                .0,
            "╢"
        );
        let space: String = format!(
            "{}{}{}",
            "║",
            "\0 \0\0 \0\0 \0\0 \0\0 \0\0 \0\0 \0│"
                .repeat(self.columns)
                .split_at(self.columns * 24 - 3)
                .0,
            "║"
        );

        let bottom: String = format!(
            "{}{}{}",
            "╚",
            "═══════╧"
                .repeat(self.columns)
                .split_at(self.columns * 24 - 3)
                .0,
            "╝"
        );

        println!("{}", top);
        let mut row = 0;
        loop {
            let mut col = 0;
            println!("{}", space);
            print!("║");
            loop {
                print!("{:^7}", self.board[self.columns * row + col]);
                col += 1;
                if col == self.columns {
                    break;
                }
                print!("│");
            }
            println!("║");
            println!("{}", space);
            row += 1;
            if row == self.rows {
                break;
            }
            println!("{}", mid);
        }
        println!("{}", bottom);
    }
}

fn shift_right(board: Vec<i32>, rows: usize, columns: usize) {}




*/
