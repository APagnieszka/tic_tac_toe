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

    pub fn make_move(&mut self, index: usize, cell: Cell) -> bool {
        if index < 9 && self.fields[index] == Cell::Empty {
            self.fields[index] = cell;
            return true;
        }

        return false;
    }

    pub fn check_winner(&self) -> Option<Cell> {
        let winning_combinations = [
            [0, 1, 2], // Row 1
            [3, 4, 5], // Row 2
            [6, 7, 8], // Row 3
            [0, 3, 6], // Column 1
            [1, 4, 7], // Column 2
            [2, 5, 8], // Column 3
            [0, 4, 8], // Diagonal 1
            [2, 4, 6], // Diagonal 2
        ];

        for combination in winning_combinations.iter() {
            let [a, b, c] = *combination;
            if self.fields[a] != Cell::Empty
                && self.fields[a] == self.fields[b]
                && self.fields[a] == self.fields[c]
            {
                return Some(self.fields[a]);
            }
        }
        None
    }

    pub fn display_winner(&self) {
        match self.check_winner() {
            Some(Cell::X) => println!("Player X wins!"),
            Some(Cell::O) => println!("Player O wins!"),
            _ => println!("No winner yet."),
        }
    }

    pub fn is_full(&self) -> bool {
        self.fields.iter().all(|&cell| cell != Cell::Empty)
    }
}
