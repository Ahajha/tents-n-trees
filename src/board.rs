use std::fmt::Display;

use grid::Grid;

#[derive(Clone, PartialEq)]
pub enum Cell {
    Tent,
    Tree,
    None,
    Unknown,
}

impl Cell {
    pub fn as_char(&self) -> char {
        match self {
            Cell::Tent => '^',
            Cell::Tree => 'T',
            Cell::None => '_',
            Cell::Unknown => '?',
        }
    }
}

pub struct TentsAndTreesBoard {
    board: Grid<Cell>,
    row_hints: Vec<u8>,
    col_hints: Vec<u8>,
}

impl TentsAndTreesBoard {
    /// tree_positions are provided as (col, row)
    /// The top left is 0, 0.
    /// (If this is undesired, can be adjusted by unpacking and re-packing the indexes upon lookup.)
    pub fn new(row_hints: Vec<u8>, col_hints: Vec<u8>, tree_positions: &[(usize, usize)]) -> Self {
        let mut board = Grid::init(row_hints.len(), col_hints.len(), Cell::Unknown);
        for pos in tree_positions {
            board[*pos] = Cell::Tree;
        }
        Self {
            board,
            row_hints,
            col_hints,
        }
    }

    fn fill_zeroes(&mut self) {
        // Start by filling in rows and columns with 0s as hints.
        let zero_rows = self
            .row_hints
            .iter()
            .enumerate()
            .filter_map(|(index, &hint)| if hint == 0 { Some(index) } else { None });

        for row_num in zero_rows {
            let row_cells = self.board.iter_row_mut(row_num as usize);

            for cell in row_cells {
                if *cell == Cell::Unknown {
                    *cell = Cell::None;
                }
            }
        }

        let zero_cols = self
            .col_hints
            .iter()
            .enumerate()
            .filter_map(|(index, &hint)| if hint == 0 { Some(index) } else { None });

        for col_num in zero_cols {
            println!("{}", col_num);
            let col_cells = self.board.iter_col_mut(col_num as usize);

            for cell in col_cells {
                if *cell == Cell::Unknown {
                    *cell = Cell::None;
                }
            }
        }
    }

    pub fn solve(&mut self) {
        self.fill_zeroes();
    }
}

impl Display for TentsAndTreesBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;

        for hint in &self.col_hints {
            write!(f, " {}", hint)?;
        }
        writeln!(f, "")?;

        for (row_num, &hint) in self.row_hints.iter().enumerate() {
            write!(f, "{}", hint)?;
            for cell in self.board.iter_row(row_num as usize) {
                write!(f, " {}", cell.as_char())?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}
