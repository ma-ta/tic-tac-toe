#![allow(dead_code)]

mod board;

use board::*;

fn main() -> Result<(), String> {
    const BOARD_SIZE: usize = 4;

    let print_setup = PrintSetup {
        player_x_o: ['⚪', '⚫'],
        empty: '\u{3000}',
        cell_width: 4
    };

    let mut board =
        match Board::new(BOARD_SIZE, BOARD_SIZE) {
            Ok(board) => board,
            _ => return Err("Board::Error".to_string())
        };


    println!("{}", board.is_cell_empty(0, 0));
    println!("board empty: {}", board.is_empty());
    board.set(0, 0, Cell::Player(1));
    println!("{}", board.is_cell_empty(0, 0));
    println!("board empty: {}", board.is_empty());
    board.set(1, 2, Cell::Player(1));
    println!("{}", board.is_cell_empty(1, 2));
    board.set(3, 1, Cell::Player(0));
    board.set(3, board.get_size().1 - 1, Cell::Player(0));
    board.print(&print_setup);
    println!("{}", board);
    for row in 0..board.get_size().0 {
        for cell in board.row_iter(row) {
            print!("{:?}|", cell);
        }
        println!();
    }
    board.clear();
    println!();
    println!("{}", board.is_cell_empty(0, 0));
    println!("{}", board.is_cell_empty(1, 2));
    println!("board empty: {}", board.is_empty());
    println!("{}", board);

    Ok(())
}
