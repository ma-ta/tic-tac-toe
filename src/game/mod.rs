//! Tic-Tac-Toe game module

pub mod board;
pub mod rules;

use board::{Cell};
pub use board::{PrintSetup, Board};
pub use rules::{GameState};

/// Instance of a Tic-Tac-Toe game.
#[derive(Debug, Clone)]
pub struct Game {
    /// Game board instance.
    board: Board,
    /// Number of players.
    players: u8,
    /// Length of line required to win.
    sequence: usize,
    /// Player whose turn it is.
    current_turn: u8,
    /// Current game state.
    state: GameState,
}

impl Game {
    /// Creates a new game.
    /// 
    /// `board`: the playing board  
    /// `players`: number of players  
    /// `sequence`: length of line required to win
    pub fn new(board: Board, players: u8, sequence: usize) -> Self {
        Self {
            board,
            players,
            sequence,
            ..Default::default()
        }
    }

    /// Returns a reference to the board of this game.
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    /// Returns the player whose turn it is.
    pub fn get_current_turn(&self) -> u8 {
        self.current_turn
    }

    /// Resets the game to its initial state.
    pub fn reset(&mut self) {
        self.board.clear();
        self.current_turn = 0;
        self.state = GameState::InProgress
    }

    /// Attempts to make a move at the given position.
    ///
    /// Returns `true` if the move was successful.
    pub fn turn(&mut self, pos: (usize, usize)) -> bool {
        if let GameState::InProgress = self.state {
            if self.make_move(pos, Cell::Player(self.current_turn)) {
                self.current_turn = if self.current_turn >= self.players - 1 { 0 }
                                    else { self.current_turn + 1 };
                self.state = rules::check_state(&self.board, self.sequence);
                return true
            }
        }

        false
    }

    /// Returns the current game state.
    pub fn get_state(&self) -> GameState {
        self.state.clone()
    }

    /* PRIVATE */

    /// Tries to make a move.
    fn make_move(&mut self, pos: (usize, usize), val: Cell) -> bool {
        if rules::is_legal_move(&self.board, pos) {
            self.board.set(pos.0, pos.1, val)
        }
        else {
            false
        }
    }
}

/// Creates a new game.
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
