use rand::prelude::*;
use rand::Rng;

use super::utils::*;
use super::Board;
use super::Direction;

pub struct Gamemode {
    pub step: fn(board: &mut Board, dir: Direction) -> (bool, i64),
    pub add_block: fn(board: &mut Board) -> bool,
}

pub const NORMAL: Gamemode = Gamemode {
    add_block: |board| {
        let mut seen = (0..board.state.len() as i32).collect::<Vec<i32>>();
        let mut rng = thread_rng();

        while seen.len() > 0 {
            let index = rng.gen_range(0..seen.len());
            let spot = seen[index] as usize;
            if board.state[spot] == 0 {
                board.state[spot] = [(2, 9), (4, 1)]
                    .choose_weighted(&mut rng, |z| z.1)
                    .unwrap()
                    .0;
                return true;
            }

            seen.remove(index);
        }

        false
    },
    step: |board, dir| {
        let mut movescore = 0;
        // Transpose/reverse to make it the same direction for all.
        match dir {
            Direction::Up => {
                transpose(board);
                reverse(board)
            }
            Direction::Down => transpose(board),
            Direction::Right => {}
            Direction::Left => reverse(board),
        }

        let mut changed = false;

        // Compress all the values to the right, to make merging easier.
        changed |= compress(board);

        // Merge all of the rows
        for r in 0..board.rows {
            let mut c: i32 = board.cols as i32 - 1;
            while c > 0 {
                let ind = board.to_index(r, c as usize);
                if board.state[ind - 1] == board.state[ind] && board.state[ind] != 0 {
                    board.state[ind] *= 2;
                    movescore += board.state[ind];
                    board.state[ind - 1] = 0;
                    c -= 1;
                    changed = true;
                }
                c -= 1
            }
        }

        changed |= compress(board);

        // Invert changes made earlier.
        match dir {
            Direction::Up => {
                reverse(board);
                transpose(board);
            }
            Direction::Down => transpose(board),
            Direction::Right => {}
            Direction::Left => reverse(board),
        }

        (changed, movescore as i64)
    },
};
