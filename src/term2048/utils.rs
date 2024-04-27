use super::Board;

pub fn transpose(board: &mut Board) {
    let mut new_state: Vec<i32> = vec![];
    for c in 0..board.cols {
        for r in 0..board.rows {
            new_state.push(board.state[r * board.cols + c]);
        }
    }
    board.state = new_state;
    let tempcols = board.cols;
    board.cols = board.rows;
    board.rows = tempcols;
}

pub fn reverse(board: &mut Board) {
    let mut new_state: Vec<i32> = vec![];
    for r in 0..board.rows {
        for c in (0..board.cols).rev() {
            new_state.push(board.state[r * board.cols + c]);
        }
    }
    board.state = new_state;
}

/// Compresses all of the blocks to the right, removing all the zeros.
/// Returns `true` if anything has changed, otherwise returns `false`.
/// ## Arguments
/// * `board` - A mutable reverence to the board to operate on.
pub fn compress(board: &mut Board) -> bool {
    let mut changed = false;
    for r in 0..board.rows {
        let mut start = board.to_index(r, board.cols - 1) as i32;
        for c in (0..board.cols).rev() {
            let ind = board.to_index(r, c);
            if board.state[ind] != 0 {
                board.state[start as usize] = board.state[ind];
                if start as usize != ind {
                    board.state[ind] = 0;
                    changed = true;
                }
                start -= 1;
            }
        }
    }
    changed
}
