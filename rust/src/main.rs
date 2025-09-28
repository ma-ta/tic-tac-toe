#![allow(dead_code)]

mod board;

use board::*;

fn main() -> Result<(), String> {
    const BOARD_SIZE: usize = 4;
    let mut _print_setup = PrintSetup::default();

    let mut board =
        match Board::new(BOARD_SIZE, BOARD_SIZE) {
            Ok(board) => board,
            Err(_) => return Err("Board::Error".to_string())
        };

    let mut count = 0;
    for row in 0..board.get_size().0 {
        for col in 0..board.get_size().1 {
            board.set(row, col, Cell::Player(count));
            count += 1;
        }
    }

    println!("{board:?}");

    let rows = board.all_rows();
    let cols = board.all_cols();
    let diags = board.all_diags();
    println!("rows:\n{rows:?}");
    println!("cols:\n{cols:?}");
    println!("diags:\n{diags:?}");

    Ok(())
}
