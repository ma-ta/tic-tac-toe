#![allow(dead_code)]

mod game;

use game::*;
use std::io::{self, Write};

fn main() {
    let mut game = Game::default();
    let mut input = String::new();

    game.board().print(&PrintSetup { ..Default::default() });
    loop {
        print!("Player {} (row col) > ", game.current_turn());
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let pos: (usize, usize) = {
            let mut iter = input
                .trim()
                .split_whitespace() 
                .map(|s| s.parse::<usize>().expect("Invalid number"));

            (iter.next().unwrap(), iter.next().unwrap())
        };
        input.clear();
        print!("{pos:?} ");
        if game.turn(pos) {println!("ok")} else {println!("err")};
        game.board().print(&PrintSetup { ..Default::default() });
        match game.state() {
            GameState::Draw => {
                println!("It's a draw!");
                break;
            }
            GameState::Win(player) => {
                println!("Player {} won!", player);
                break;
            }
            _ => continue
        }
    }
}
