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
        "| (c) 2025-26  by Ma-TA         |",
        "+-------------------------------+"
    );
    const PAUSE: u64 = 2;
    let mut game = Game::default();
    let mut rng = rand::rng();

    loop {
        print!("\x1B[2J\x1B[H");
        println!("{title}");
        println!("(Ctrl+C to quit)\n");
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
            let mut input = String::new();
            let mut nums: [usize; 2] = [0, 0];

            'io_input:
            loop {
                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let mut iter = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<usize>());

                for i in 0..nums.len() {
                    match iter.next() {
                        None => {
                            print!("Enter two numbers (e.g. 1 1) > ");
                            let _ = io::stdout().flush();
                            continue 'io_input;
                        }
                        Some(num) => {
                            match num {
                                Err(_) => {
                                    print!("Enter numbers only (e.g. 1 1) > ");
                                    let _ = io::stdout().flush();
                                    continue 'io_input;
                                }
                                Ok(n) => {
                                    nums[i] = n;
                                }
                            }
                        }
                    }
                }
                break;
            }
            pos.0 = nums[0];
            pos.1 = nums[1];
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
                println!("IT'S A DRAW!\n");
                break;
            }
            GameState::Win(player) => {
                print!("\x1B[2J\x1B[H");
                println!("{title}");
                game.get_board().print(&PrintSetup { ..Default::default() });
                println!("PLAYER {} WON!\n", player);
                break;
            }
            _ => continue
        }
    }
}
