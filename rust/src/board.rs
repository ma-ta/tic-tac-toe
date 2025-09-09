use std::{
    fmt, ops::{Deref, DerefMut}
};

pub enum Error {
    MinSize(String),
}

#[derive(Debug, Clone)]
pub struct Board {
    size: (usize, usize),
    cells: Cells,
}

impl Board {
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

    pub fn clear(&mut self) {
        self.cells.fill(Cell::Empty);
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        if self.cells.iter().all(Cell::is_empty) {
            return true;
        }
        false
    }

    pub fn is_cell_empty(&self, row: usize, col: usize) -> bool {
        if let Some(cell) = self.get(row, col) {
            return cell.is_empty()
        }
        false
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
        let index = self.idx(row, col);

        self.cells.get(index).copied()
    }

    pub fn row_iter(&self, row: usize) -> impl Iterator<Item = &Cell> {
        let start = row * self.size.1;
        let end = start + self.size.1;
        self.cells[start..end].iter()
    }
/*
    pub fn col_iter(&self, row: usize) -> impl Iterator<Item = &Cell> {
        
    }
*/
    pub fn set(&mut self, row: usize, col: usize, status: Cell) -> bool {
        let index = self.idx(row, col);

        if let Some(cell) = self.cells.get_mut(index) {
            if *cell == Cell::Empty {
                *cell = status;
                return true
            }
        }
        false
    }

    pub fn print(&self) {
        println!("{self}");
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.size.1 + col
    }
}

impl fmt::Display for Board {
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
                        Cell::Player(0) => 'X',
                        Cell::Player(1) => 'O',
                        Cell::Player(n) => (n + b'0') as char
                    }
                )?;
            }
            writeln!(f)?;
        }
        write!(f, "{}+", "+---".repeat(self.size.1))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Player(u8),
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        *self == Cell::Empty
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Cell::Empty => '_',
            Cell::Player(0) => 'X',
            Cell::Player(1) => 'O',
            Cell::Player(n) => (*n + b'0') as char
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
