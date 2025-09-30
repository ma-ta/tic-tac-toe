//! Tic-tac-toe rules for the generic game board.

use super::*;

/// Actual game state.
#[derive(Debug, Clone)]
pub enum GameState {
    Win(u8),
    Draw,
    InProgress,
}

impl Board {
    /// Returns a vector of all possible moves.
    pub fn get_legal_moves(&self) -> Option<Vec<(usize, usize)>> {
        if self.is_full() {
            None
        }
        else {
            let mut ret = Vec::new();

            for row in 0..self.get_size().0 {
                for col in 0..self.get_size().1 {
                    if let Some(Cell::Empty) = self.get(row, col) {
                        ret.push((row, col));
                    }
                }
            }
            Some(ret)
        }
    }

    /// Checks if the move is legal.
    pub fn is_legal_move(&self, pos: (usize, usize)) -> bool {
        self.is_cell_empty(pos.0, pos.1)
    }

    /// Tries to make a move.
    pub fn make_move(&mut self, pos: (usize, usize), val: Cell) -> bool {
        if self.is_cell_empty(pos.0, pos.1) {
            self.set(pos.0, pos.1, val)
        }
        else {
            false
        }
    }

    /// Checks the actual state of the game.
    pub fn check_state(&self, sequence: usize) -> GameState {
        let mut lines = self.all_rows();
        lines.extend(self.all_cols());
        lines.extend(self.all_diags());

        for line in lines {
            let mut count = 0;
            let mut last_player: Option<u8> = None;

            for cell in line {
                if let Cell::Player(n) = cell {
                    if Some(*n) == last_player {
                        count += 1;
                    } else {
                        count = 1;
                        last_player = Some(*n);
                    }

                    if count == sequence {
                        return GameState::Win(*n);
                    }
                } else {
                    count = 0;
                    last_player = None;
                }
            }
        }

        if self.is_full() {
            GameState::Draw
        }
        else {
            GameState::InProgress
        }
    }

    /// Some simple tests.
    pub fn test_rules() {
        let mut board = Board::default();
        board.set(0, 0, Cell::Player(0));
        board.set(0, 1, Cell::Player(0));
        board.set(0, 2, Cell::Player(1));
        board.set(1, 0, Cell::Player(1));
        board.set(1, 1, Cell::Player(1));
        board.set(1, 2, Cell::Player(0));
        board.set(2, 0, Cell::Player(0));
        board.set(2, 1, Cell::Player(0));
        let r#move = (2, 2);
        println!("make_move({move:?}): {:?}", board.make_move(r#move, Cell::Player(1)));
        println!("make_move({move:?}): {:?}", board.make_move(r#move, Cell::Player(0)));

        board.print(&PrintSetup { ..Default::default() });
        println!("{:?}", board.check_state(3));
    }
}
