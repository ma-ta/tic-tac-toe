#![allow(dead_code)]

mod board;  use std::cell;

use board::*;

fn main() -> Result<(), String> {
    const BOARD_SIZE: usize = 4;

    let _print_setup = PrintSetup {
        player_x_o: ['⚪', '⚫'],
        empty: '\u{3000}',
        cell_width: 4
    };

    let mut board =
        match Board::new(BOARD_SIZE, BOARD_SIZE) {
            Ok(board) => board,
            _ => return Err("Board::Error".to_string())
        };

    let mut j = 0;
    for i in 0..4 {
        for cell in board.row_iter(i) {
            *cell = Cell::Player(j);
            j += 1;
        }
    }

    println!("{board:?}");

    let iter = board.col_iter(3);
    for cell in iter {
        println!("{cell:?}");
    }

    Ok(())
}
