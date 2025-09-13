use std::{
    fmt, ops::{Deref, DerefMut}
};

pub enum Error {
    MinSize(String),
}

#[derive(Clone)]
pub struct Board {
    size: (usize, usize),
    cells: Cells,
}

pub struct PrintSetup {
    pub player_x_o: [char; 2],
    pub empty: char,
    pub cell_width: u8
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

    pub fn row_iter(&mut self, row: usize) -> impl Iterator<Item = &mut Cell> {
        let start = row * self.size.1;
        let end = start + self.size.1;
        self.cells[start..end].iter_mut()
    }

    pub fn col_iter(&mut self, col: usize) -> impl Iterator<Item = &mut Cell> {
        let start = col;
        let end = self.cells.len() - self.size.1 + 1 + col;
        self.cells[start..end].iter_mut().step_by(self.size.1)
    }

    pub fn set(&mut self, row: usize, col: usize, status: Cell) -> bool {
        let index = self.idx(row, col);

        if let Some(cell) = self.cells.get_mut(index) {
            cell.set(status);
            return true;
        }
        false
    }

    pub fn print(&self, setup: &PrintSetup) {
        let size = self.get_size();
        let mut out = String::new();

        for row in 0..size.0 {
            out += "+";
            out += &format!("{}+", "-".repeat(setup.cell_width as usize)).repeat(self.size.1);
            out += "\n|";
            for col in 0..size.1 {
                out += &format!(
                    " {} |",
                    match self.get(row, col).unwrap_or(Cell::Empty) {
                        Cell::Empty => setup.empty,
                        Cell::Player(0) => setup.player_x_o[0],
                        Cell::Player(1) => setup.player_x_o[1],
                        Cell::Player(n) => (n + b'0' + 1) as char
                    }
                );
            }
            out += "\n";
        }
        out += "+";
        out += &format!("{}+", "-".repeat(setup.cell_width as usize)).repeat(self.size.1);

        println!("{out}");
    }

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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
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
