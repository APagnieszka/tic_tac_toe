use crate::cell::Cell;

#[derive(Clone, Copy)]
pub struct Board {
    pub fields: [[Cell; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Self {
            fields: [[Cell::Empty; 3]; 3],
        }
    }

    pub fn print(board: &Board) {
        for row in board.fields.iter() {
            for cell in row.iter() {
                match cell {
                    Cell::Empty => print!("_"),
                    Cell::X => print!("X"),
                    Cell::O => print!("O"),
                }
            }
            println!();
        }
    }
}