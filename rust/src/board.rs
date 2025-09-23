//! A generic game board for tic-tac-toe and similar games.

use std::{
    fmt, ops::{Deref, DerefMut}
};

/// Error types used by the board module.
pub enum Error {
    /// The board size is too small.
    MinSize(String),
}

/// A two-dimensional game board.
#[derive(Clone)]
pub struct Board {
    /// Board dimensions: (rows, columns).
    size: (usize, usize),
    /// Board cells stored in a 1D vector.
    cells: Cells,
}

/// Configuration for how the board is printed.
pub struct PrintSetup {
    /// Symbols for the players (e.g. ['X', 'O']).
    pub players: Vec<char>,
    /// Width of each cell in characters.
    pub cell_width: usize,
}

/// Returns a new PrintSetup with default values
impl Default for PrintSetup {
    fn default() -> Self {
        Self {
            players: vec!['⚪', '⚫'],
            cell_width: 4
        }
    }
}

impl Board {
    /// Creates a new game board with the given dimensions.
    pub fn new(rows: usize, cols: usize) -> Result<Self, Error> {
        const MIN_SIZE: (usize, usize) = (3, 3);

        if rows >= MIN_SIZE.0 && cols >= MIN_SIZE.1 {
            Ok(Self {
                size: (rows, cols),
                cells: Cells::new(rows, cols),
            })
        } else {
            Err (
                Error::MinSize(
                    format!(
                        "Board size must be at least {}x{}",
                        MIN_SIZE.0, MIN_SIZE.1
                    )
                )
            )
        }
    }

    /// Resets all cells to empty.
    pub fn clear(&mut self) {
        self.cells.fill(Cell::Empty);
    }

    /// Returns the board dimensions as (rows, columns).
    pub fn get_size(&self) -> (usize, usize) {
        self.size
    }

    /// Returns `true` if all cells are empty.
    pub fn is_empty(&self) -> bool {
        self.cells.iter().all(Cell::is_empty)
    }

    /// Returns `true` if the given cell is empty.
    pub fn is_cell_empty(&self, row: usize, col: usize) -> bool {
        if let Some(cell) = self.get(row, col) {
            return cell.is_empty()
        }
        false
    }

    /// Returns the content of the given cell, or `None` if out of bounds.
    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
        let index = self.idx(row, col);

        self.cells.get(index).copied()
    }

    /// Returns an iterator over the given row.
    pub fn row_iter(&self, row: usize) -> impl Iterator<Item = &Cell> {
        let start = row * self.size.1;
        let end = start + self.size.1;
        self.cells[start..end].iter()
    }

    /// Returns an iterator over the given column.
    pub fn col_iter(&self, col: usize) -> impl Iterator<Item = &Cell> {
        self.cells[col..].iter().step_by(self.size.1)
    }

    /// Returns a vector of all rows
    pub fn all_rows(&self) -> Vec<Vec<Cell>> {
        let mut ret: Vec<Vec<_>> = Vec::new();
        for row in 0..self.size.0 {
            ret.push(self.row_iter(row).copied().collect());
        }
        ret
    }

    /// Returns a vector of all columns
    pub fn all_colls(&self) -> Vec<Vec<Cell>> {
        let mut ret: Vec<Vec<_>> = Vec::new();
        for col in 0..self.size.1 {
            ret.push(self.col_iter(col).copied().collect());
        }
        ret
    }

    /// Attempts to set a cell to the given value.
    /// Returns `true` if the operation succeeded.
    pub fn set(&mut self, row: usize, col: usize, status: Cell) -> bool {
        let index = self.idx(row, col);

        if let Some(cell) = self.cells.get_mut(index) {
            cell.set(status);
            return true;
        }
        false
    }

    /// Pretty-prints the board using the given setup.
    pub fn print(&self, setup: &PrintSetup) {
        let size = self.get_size();
        let mut out = String::new();

        for row in 0..size.0 {
            out += "+";
            out += &format!("{}+", "-".repeat(setup.cell_width)).repeat(self.size.1);
            out += "\n|";
            for col in 0..size.1 {
                out += &format!(
                    " {} |",
                    match self.get(row, col).unwrap_or(Cell::Empty) {
                        Cell::Empty => {
                            " ".repeat(setup.cell_width / 2)
                        },
                        Cell::Player(n) => {
                            if n as usize >= setup.players.len() {
                                ((n + b'0' + 1) as char).to_string()
                                + &" ".repeat(setup.cell_width - 3)
                            }
                            else {
                                setup.players[n as usize].to_string()
                            }
                        }
                    }
                );
            }
            out += "\n";
        }
        out += "+";
        out += &format!("{}+", "-".repeat(setup.cell_width)).repeat(self.size.1);

        println!("{out}");
    }

    /// Converts a (row, col) pair into a 1D index.
    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.size.1 + col
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.get_size();

        for row in 0..size.0 {
            write!(f, "{}+\n|", "+---".repeat(size.1))?;
            for col in 0..size.1 {
                write!(
                    f,
                    " {} |",
                    match self.get(row, col).unwrap_or(Cell::Empty) {
                        Cell::Empty => ' ',
                        Cell::Player(n) => (n + b'0') as char
                    }
                )?;
            }
            writeln!(f)?;
        }
        write!(f, "{}+", "+---".repeat(self.size.1))
    }
}

/// A single cell on the game board.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    /// The cell is empty.
    Empty,
    /// The cell is occupied by the given player (0 = first player, 1 = second, etc.).
    Player(u8),
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        *self == Cell::Empty
    }

    pub fn set(&mut self, status: Cell) -> bool {
        if *self == Cell::Empty {
            *self = status;
            true
        }
        else {
            false
        }
    }

    pub fn get(&self) -> Cell {
        *self
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Cell::Empty => '_',
            Cell::Player(n) => (*n + b'0') as char
        };
        write!(f, "{}", symbol)
    }
}

/// Internal wrapper around a 1D vector of cells.
#[derive(Debug, Clone)]
struct Cells(Vec<Cell>);

impl Cells {
    fn new(rows: usize, cols: usize) -> Self {
        Self(vec![Cell::Empty; rows * cols])
    }
}

impl Deref for Cells {
    type Target = Vec<Cell>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cells {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
