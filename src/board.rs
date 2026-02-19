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

    fn minimax(&mut self, depth: usize, is_maximizing: bool) -> i32 {
        if let Some(winner) = self.check_winner() {
            return match winner {
                Cell::X => 10 + depth as i32,  // faster wins are better
                Cell::O => -10 - depth as i32, // slower losses are better
                Cell::Empty => 0,
            };
        }

        if self.is_full() {
            return 0;
        }

        if is_maximizing {
            let mut best_score = i32::MIN;
            for i in 0..9 {
                if self.fields[i] == Cell::Empty {
                    self.fields[i] = Cell::X; // Do move for AI
                    let score = self.minimax(depth + 1, false);
                    self.fields[i] = Cell::Empty; // Undo move
                    best_score = best_score.max(score);
                }
            }
            best_score
        } else {
            let mut best_score = i32::MAX;
            for i in 0..9 {
                if self.fields[i] == Cell::Empty {
                    self.fields[i] = Cell::O; // Do move for opponent
                    let score = self.minimax(depth + 1, true);
                    self.fields[i] = Cell::Empty; // Undo move
                    best_score = best_score.min(score);
                }
            }
            best_score
        }
    }

    pub fn find_best_move(&mut self) -> Option<usize> {
        let mut best_score = i32::MIN;
        let mut best_move = None;

        for i in 0..9 {
            if self.fields[i] == Cell::Empty {
                self.fields[i] = Cell::X; // Do move for AI
                let score = self.minimax(0, false);
                self.fields[i] = Cell::Empty; // Undo move

                if score > best_score {
                    best_score = score;
                    best_move = Some(i);
                }
            }
        }
        best_move
    }
}
