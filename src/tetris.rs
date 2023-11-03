use std::io::{stdout, Write};

use crossterm::execute;
use crossterm::style::Color;

use crate::tiles;
use crate::tiles::Tiles;

#[derive(Debug)]
pub(crate) struct Canvas {
    game_board: [[Option<Color>; 10]; 20],
    score: u32,
    tiles_vec: Vec<Tiles>,
}

impl Canvas {
    pub(crate) fn new() -> Canvas {
        Canvas {
            game_board: [[None; 10]; 20],
            score: 0,
            tiles_vec: tiles::get_tiles_vec(),
        }
    }

    pub(crate) fn new_tail(&mut self, tail_index: usize, type_index: usize) {
        let tile = &self.tiles_vec[tail_index];
        let tail_color = tile.color;
        let tail_content = tile.content[type_index];

        for (row_index, row) in self.game_board.iter_mut().enumerate() {
            for col_index in 0..row.len() {
                if row_index < 4 && col_index < 4 {
                    if tail_content[row_index][col_index] == 1 {
                        row[col_index] = Some(tail_color);
                    }
                }
            }
        }
    }

    pub(crate) fn refresh_line(&self, lines: Vec<u16>) {
        let stdout = &mut stdout();
        for row_index in lines {
            execute!(stdout, crossterm::cursor::MoveTo(0, row_index)).unwrap();
            let row_data = self.game_board[row_index as usize];
            for content in row_data {
                match content {
                    None => {
                        execute!(stdout, crossterm::style::SetBackgroundColor(Color::White))
                            .unwrap();
                        print!("  ");
                    }
                    Some(color) => {
                        execute!(stdout, crossterm::style::SetBackgroundColor(color)).unwrap();
                        print!("  ");
                    }
                }
            }
            stdout.flush().unwrap();
        }
    }
}
