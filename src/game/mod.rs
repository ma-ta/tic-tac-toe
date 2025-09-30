mod board;

use board::*;
pub use board::{PrintSetup, rules::GameState};

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    players: u8,
    sequence: usize,
    current_turn: u8,
    state: GameState,
}

impl Game {
    pub fn new(board: Board, players: u8, sequence: usize) -> Self {
        Self {
            board,
            players,
            sequence,
            ..Default::default()
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_turn(&self) -> u8 {
        self.current_turn
    }

    pub fn reset(&mut self) {
        self.board.clear();
        self.current_turn = 0;
        self.state = GameState::InProgress
    }

    pub fn turn(&mut self, pos: (usize, usize)) -> bool {
        if let GameState::InProgress = self.state {
            if self.board.make_move(pos, Cell::Player(self.current_turn)) {
                self.current_turn = if self.current_turn >= self.players - 1 { 0 }
                                    else { self.current_turn + 1 };
                self.state = self.board.check_state(self.sequence);
                return true
            }
        }

        false
    }

    pub fn state(&self) -> GameState {
        self.state.clone()
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: Board::default(),
            players: 2,
            sequence: 3,
            current_turn: 0,
            state: GameState::InProgress
        }
    }
}
