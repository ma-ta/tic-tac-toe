#![allow(dead_code)]

mod board;

use board::*;

fn main() -> Result<(), String> {
    const BOARD_SIZE: usize = 4;

    let mut board =
        match Board::new(BOARD_SIZE, BOARD_SIZE) {
            Ok(board) => board,
            Err(Error::MinSize(msg, _)) => return Err(msg)
        };

    board.set(0, 0, Cell::Player2);
    board.set(1, 2, Cell::Player2);
    board.set(3, 1, Cell::Player1);
    board.set(3, board.get_size().1 - 1, Cell::Player1);

    board.print();
    board.clear();
    board.print();

    Ok(())
}
