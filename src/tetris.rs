use std::io::{stdout, Write};

use crossterm::execute;

use crate::tiles;
use crate::tiles::Tiles;

#[derive(Debug)]
pub(crate) struct Canvas {
    game_board: [[u8; 20]; 20],
    score: u32,
    tiles_vec: Vec<Tiles>,
}

impl Canvas {
    pub(crate) fn new() -> Canvas {
        Canvas {
            game_board: [[0; 20]; 20],
            score: 0,
            tiles_vec: tiles::get_tiles_vec(),
        }
    }

    pub(crate) fn show(&self) {
        let stdout = &mut stdout();
        for outer in 1..3 {
            for i in 0..10 {
                execute!(stdout, crossterm::cursor::MoveTo(i * 2, outer),).unwrap();
            }
        }
        stdout.flush().unwrap();
        execute!(
            stdout,
            // 渲染内容
        )
        .unwrap();
    }

    pub(crate) fn run() {
        loop {
            // 延时10毫秒
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
