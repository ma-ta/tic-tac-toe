use std::{
    fmt,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone)]
pub struct Board {
    size: (usize, usize),
    cells: Cells,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Result<Self, String> {
        const MIN_SIZE: (usize, usize) = (1, 3);
        if rows >= MIN_SIZE.0 && cols >= MIN_SIZE.1 {
            Ok(Self {
                size: (rows, cols),
                cells: Cells::new(rows, cols),
            })
        } else {
            Err(
                format!(
                    "Board size must be at least {}x{}",
                    MIN_SIZE.0, MIN_SIZE.1
                )
            )
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.size
    }

    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        matches!(self.get(row, col), Some(Cell::Empty))
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
        let index = self.get_index(row, col);
        
        self.cells.get(index).copied()
    }

    pub fn set(&mut self, row: usize, col: usize, status: Cell) -> bool {
        let index = self.get_index(row, col);

        if let Some(cell) = self.cells.get_mut(index) {
            *cell = status;
            return true;
        }
        
        false
    }

    pub fn print(&self) {
        println!("{self}");
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.size.1 + col
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let size = self.get_size();

        for row in 0..size.0 {
            out += &format!("{}+", "+----".repeat(size.1));
            out += "\n|";
        for col in 0..size.1 {
            out += &format!(" {} |", self.get(row, col).unwrap());
        }
        out += "\n";
    }
        out += &format!("{}+", "+----".repeat(self.size.1));

        write!(f, "{out}")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Player1, // X
    Player2, // O
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Cell::Empty => '\u{3000}',
            Cell::Player1 => '⚪', // X (⚪)
            Cell::Player2 => '⚫', // O (⚫)
        };
        write!(f, "{}", symbol)
    }
}

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
