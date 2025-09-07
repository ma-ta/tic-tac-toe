#![allow(dead_code)]

mod board;

use board::*;

fn main() -> Result<(), String> {
    const BOARD_SIZE: usize = 4;

    let mut board =
        match Board::new(BOARD_SIZE, BOARD_SIZE) {
            Ok(board) => board,
            _ => return Err("Board::Error".to_string())
        };

    println!("{}", board.is_cell_empty(0, 0));
    println!("board empty: {}", board.is_empty());
    board.set(0, 0, Cell::Player2);
    println!("{}", board.is_cell_empty(0, 0));
    println!("board empty: {}", board.is_empty());
    board.set(1, 2, Cell::Player2);
    println!("{}", board.is_cell_empty(1, 2));
    board.set(3, 1, Cell::Player1);
    board.set(3, board.get_size().1 - 1, Cell::Player1);
    board.print();
    board.clear();
    println!("{}", board.is_cell_empty(0, 0));
    println!("{}", board.is_cell_empty(1, 2));
    println!("board empty: {}", board.is_empty());
    board.print();

    Ok(())
}
