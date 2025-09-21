#![allow(dead_code)]

mod board;

use board::*;

fn main() -> Result<(), String> {
    const BOARD_SIZE: usize = 4;
    let mut _print_setup = PrintSetup {
        players: vec!['X', 'O'],
        cell_width: 3,
    };

    let mut board =
        match Board::new(BOARD_SIZE, BOARD_SIZE) {
            Ok(board) => board,
            Err(_) => return Err("Board::Error".to_string())
        };

    board.set(1, 1, Cell::Player(0));
    board.set(1, 2, Cell::Player(1));

    board.print(&_print_setup);
    println!("{board:?}");

    Ok(())
}
