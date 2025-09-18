#![allow(dead_code)]

mod board;
//mod rules;

use board::*;

fn main() -> Result<(), String> {
    const BOARD_SIZE: usize = 4;

    let _print_setup = PrintSetup {
        players: vec!['⚪', '⚫'],
        cell_width: 4
    };

    let mut board =
        match Board::new(BOARD_SIZE, BOARD_SIZE) {
            Ok(board) => board,
            _ => return Err("Board::Error".to_string())
        };

    board.set(1, 1, Cell::Player(0));
    board.set(1, 2, Cell::Player(1));

    board.print(&_print_setup);

    Ok(())
}
