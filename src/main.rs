#![allow(dead_code)]

mod game;

use game::*;
use rand::Rng;
use std::io::{self, Write};
use std::{thread, time::Duration};


fn main() {
    let mut game = Game::default();
    let mut input = String::new();
    let mut rng = rand::rng();

    loop {
        print!("\x1B[2J\x1B[H");
        game.board().print(&PrintSetup { ..Default::default() });
        print!("Player {} (row col) > ", game.current_turn());
        let _ = io::stdout().flush();
        let pos: (usize, usize);

        if game.current_turn() != 0 {
            let moves
                = game.board().get_legal_moves().unwrap();
            pos = moves[rng.random_range(0..moves.len())];
        }
        else {
            io::stdin().read_line(&mut input).expect("Failed to read line");
            pos = {
                let mut iter = input
                    .trim()
                    .split_whitespace() 
                    .map(|s| s.parse::<usize>().expect("Invalid number"));

                (iter.next().unwrap(), iter.next().unwrap())
            };
            input.clear();
        }

        print!("{pos:?} ");
        if game.turn(pos) {println!("ok")} else {println!("err")};
        if game.current_turn() == 0 {
            thread::sleep(Duration::from_secs(2));
        }
        match game.state() {
            GameState::Draw => {
                print!("\x1B[2J\x1B[H");
                game.board().print(&PrintSetup { ..Default::default() });
                println!("It's a draw!");
                break;
            }
            GameState::Win(player) => {
                print!("\x1B[2J\x1B[H");
                game.board().print(&PrintSetup { ..Default::default() });
                println!("Player {} won!", player);
                break;
            }
            _ => continue
        }
    }
}
