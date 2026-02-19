use macroquad::prelude::*;

use crate::board::Board;
use crate::cell::Cell;

const CELL_SIZE: f32 = 120.0;
const BOARD_SIZE: f32 = CELL_SIZE * 3.0;
const OFFSET_X: f32 = 40.0;
const OFFSET_Y: f32 = 40.0;

pub struct GameWindow {
    board: Board,
    game_over: bool,
}

impl GameWindow {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            game_over: false,
        }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(WHITE);

            self.handle_input();
            self.draw_board();
            self.draw_status();

            next_frame().await;
        }
    }

    fn handle_input(&mut self) {
        if self.game_over {
            return;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if let Some(index) = Self::cell_from_mouse(mouse_x, mouse_y) {
                if self.board.make_move(index, Cell::O) {
                    self.update_game_over_state();

                    if !self.game_over {
                        if let Some(best_move) = self.board.find_best_move() {
                            self.board.make_move(best_move, Cell::X);
                        }
                        self.update_game_over_state();
                    }
                }
            }
        }
    }

    fn update_game_over_state(&mut self) {
        self.game_over = self.board.check_winner().is_some() || self.board.is_full();
    }

    fn cell_from_mouse(mouse_x: f32, mouse_y: f32) -> Option<usize> {
        if mouse_x < OFFSET_X
            || mouse_x > OFFSET_X + BOARD_SIZE
            || mouse_y < OFFSET_Y
            || mouse_y > OFFSET_Y + BOARD_SIZE
        {
            return None;
        }

        let col = ((mouse_x - OFFSET_X) / CELL_SIZE) as usize;
        let row = ((mouse_y - OFFSET_Y) / CELL_SIZE) as usize;

        Some(row * 3 + col)
    }

    fn draw_board(&self) {
        for i in 1..3 {
            let x = OFFSET_X + i as f32 * CELL_SIZE;
            draw_line(x, OFFSET_Y, x, OFFSET_Y + BOARD_SIZE, 3.0, BLACK);

            let y = OFFSET_Y + i as f32 * CELL_SIZE;
            draw_line(OFFSET_X, y, OFFSET_X + BOARD_SIZE, y, 3.0, BLACK);
        }

        for row in 0..3 {
            for col in 0..3 {
                let idx = row * 3 + col;
                let x = OFFSET_X + col as f32 * CELL_SIZE;
                let y = OFFSET_Y + row as f32 * CELL_SIZE;

                match self.board.fields[idx] {
                    Cell::X => {
                        draw_line(
                            x + 15.0,
                            y + 15.0,
                            x + CELL_SIZE - 15.0,
                            y + CELL_SIZE - 15.0,
                            5.0,
                            RED,
                        );
                        draw_line(
                            x + CELL_SIZE - 15.0,
                            y + 15.0,
                            x + 15.0,
                            y + CELL_SIZE - 15.0,
                            5.0,
                            RED,
                        );
                    }
                    Cell::O => {
                        draw_circle_lines(
                            x + CELL_SIZE / 2.0,
                            y + CELL_SIZE / 2.0,
                            CELL_SIZE / 2.5,
                            5.0,
                            BLUE,
                        );
                    }
                    Cell::Empty => {}
                }
            }
        }
    }

    fn draw_status(&self) {
        let status = match self.board.check_winner() {
            Some(Cell::X) => "Winner: X",
            Some(Cell::O) => "Winner: O",
            Some(Cell::Empty) => "",
            None if self.board.is_full() => "Draw",
            None => "Your move (O)",
        };

        draw_text(status, OFFSET_X, OFFSET_Y + BOARD_SIZE + 40.0, 32.0, DARKGRAY);
    }
}
