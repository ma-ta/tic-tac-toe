#![allow(dead_code)]

mod board;

use board::*;

fn main() {
    const SIZE: (usize, usize) = (4, 4);
    println!("Board::test_board({SIZE:?}):\n");
    let _ = Board::test_board(SIZE);
    println!("\nBoard::test_rules():\n");
    Board::test_rules();
}
