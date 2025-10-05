//! Prototype for testing the game only

#![allow(dead_code)]

mod game;

use game::*;
use rand::Rng;
use std::io::{self, Write};
use std::{thread, time::Duration};

fn main() {
    let title = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        "+-------------------------------+",
        "| Tic Tac Toe Prototype in Rust |",
        "+-------------------------------+",
        "| (c) 2025  by Ma-TA            |",
        "+-------------------------------+"
    );
    const PAUSE: u64 = 3;
    let mut game = Game::default();
    let mut input = String::new();
    let mut rng = rand::rng();

    loop {
        print!("\x1B[2J\x1B[H");
        println!("{title}");
        game.get_board().print(&PrintSetup { ..Default::default() });
        print!("> PLAYER: {} <\n(row col) > ", game.get_current_turn());
        let _ = io::stdout().flush();
        let mut pos: (usize, usize) = (0, 0);

        if game.get_current_turn() != 0 {
            let moves
                = rules::get_legal_moves(&game.get_board()).unwrap();
            pos = moves[rng.random_range(0..moves.len())];
        }
        else {
            loop {
                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let mut iter = input
                    .trim()
                    .split_whitespace() 
                    .map(|s| s.parse::<usize>());

                match iter.next().unwrap() {
                    Ok(n) => pos.0 = n,
                    _ => {
                        println!("(!) Enter two numbers (e.g. 0 1) >");
                        continue
                    }
                }
                match iter.next().unwrap() {
                    Ok(n) => pos.1 = n,
                    _ => {
                        println!("(!) Enter two numbers (e.g. 2 0) >");
                        continue
                    }
                }
                break;
            }
        }

        print!("{} {} => ", pos.0, pos.1);
        if game.turn(pos) {println!("OK")} else {println!("ERR!")};
        if game.get_current_turn() == 0 {
            thread::sleep(Duration::from_secs(PAUSE));
        }
        match game.get_state() {
            GameState::Draw => {
                print!("\x1B[2J\x1B[H");
                println!("{title}");
                game.get_board().print(&PrintSetup { ..Default::default() });
                println!("It's a draw!\n");
                break;
            }
            GameState::Win(player) => {
                print!("\x1B[2J\x1B[H");
                println!("{title}");
                game.get_board().print(&PrintSetup { ..Default::default() });
                println!("Player {} won!\n", player);
                break;
            }
            _ => continue
        }
    }
}
