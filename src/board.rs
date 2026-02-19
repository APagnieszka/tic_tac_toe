use crate::cell::Cell;

#[derive(Clone, Copy)]
pub struct Board {
    pub fields: [Cell; 9],
}

impl Board {
    pub fn new() -> Self {
        Self {
            fields: [Cell::Empty; 9],
        }
    }

    pub fn display(&self) {
        for i in 0..3 {
            for j in 0..3 {
                let cell = self.fields[i * 3 + j];
                match cell {
                    Cell::Empty => print!("_"),
                    Cell::X => print!("X"),
                    Cell::O => print!("O"),
                }
            }
            println!();
        }
    }

    pub fn make_move(&mut self, index: usize, cell: Cell)  -> bool{
        if index < 9 && self.fields[index] == Cell::Empty {
            self.fields[index] = cell;
            return true;
        }
        return false;
    }

}