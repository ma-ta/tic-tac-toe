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

    board.set(0, 3, Cell::Player(0));
    board.set(1, 1, Cell::Player(0));
    board.set(1, 2, Cell::Player(1));
    board.set(3, 3, Cell::Player(1));

    let mut board_vec: Vec<_> = board.row_iter(1).collect();
    println!("řádek 1: {board_vec:?}");
    board_vec = board.row_iter(3).collect();
    println!("řádek 3: {board_vec:?}");
    board_vec = board.col_iter(0).collect();
    println!("sloupec 0: {board_vec:?}");
    board_vec = board.col_iter(3).collect();
    println!("sloupec 3: {board_vec:?}");

    println!("{board:?}");

    let mut board_vec = board.all_rows();
    println!("řádky:\n{board_vec:?}");
    board_vec = board.all_colls();
    println!("sloupce:\n{board_vec:?}");

    let iter = board.diag_iter((1, 1));
    let cells: Vec<_> = iter.collect();
    println!("{cells:?}");

    Ok(())
}
