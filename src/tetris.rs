use std::io;
use std::io::Write;

use crossterm::execute;
use crossterm::style::Color;

use crate::tiles;
use crate::tiles::Tiles;

#[derive(Debug)]
pub(crate) struct Canvas {
    current_tile: [[Option<Color>; 4]; 4],
    pub(crate) tile_position: (i8, i8),
    game_board: [[Option<Color>; 10]; 20],
    score: u32,
    tiles_vec: Vec<Tiles>,
}

pub enum Direction {
    Left,
    Right,
    Down,
}

impl Canvas {
    pub(crate) fn new() -> Canvas {
        Canvas {
            current_tile: [[None; 4]; 4],
            tile_position: (0, 0),
            game_board: [[None; 10]; 20],
            score: 0,
            tiles_vec: tiles::get_tiles_vec(),
        }
    }

    pub(crate) fn new_tail(&mut self, tail_index: usize, type_index: usize) {
        let tile = &self.tiles_vec[tail_index];
        let tail_color = tile.color;
        let tail_content = tile.content[type_index];
        for (row_index, row) in self.current_tile.iter_mut().enumerate() {
            for col_index in 0..4 {
                if tail_content[row_index][col_index] == 1 {
                    row[col_index] = Some(tail_color);
                }
            }
        }
        // 初始化tail的位置，放在游戏区域的中上方，x:3, y:-3
        let mut y: i8 = -4;
        // 如果当前tail的最后一行没有内容，则将位置下移一行
        if tail_content[3].iter().all(|&x| x == 0) {
            y + 1;
        }
        self.tile_position = (3, y);
    }

    pub(crate) fn draw_tail(&mut self, direction: Direction) {
        let (x, y) = self.tile_position;
        // 当y轴加tail content的高度大于0时，说明方块的内容已经进入到游戏区域内
        if y + 4 > 0 {
            // 遍历tail覆盖的区域，将内容显示到终端
            let start_y = if y < 0 { 0 } else { y };
            for inter_y in start_y..y + 4 {
                let refresh_y = if y < 0 { inter_y } else { inter_y - y };
                for iter_x in 0..20 {
                    if iter_x % 2 == 0 {
                        let refresh_x = iter_x / 2;
                        if refresh_x >= x && refresh_x < x + 4 {
                            // 移动光标到指定位置
                            execute!(
                                io::stdout(),
                                crossterm::cursor::MoveTo(iter_x as u16, inter_y as u16)
                            )
                            .unwrap();
                            match self.current_tile[refresh_y as usize]
                                [refresh_x as usize - x as usize]
                            {
                                None => {
                                    execute!(
                                        io::stdout(),
                                        crossterm::style::SetBackgroundColor(Color::White)
                                    )
                                    .unwrap();
                                    write!(io::stdout(), "　").unwrap();
                                }
                                Some(color) => {
                                    execute!(
                                        io::stdout(),
                                        crossterm::style::SetBackgroundColor(color)
                                    )
                                    .unwrap();
                                    write!(io::stdout(), "　").unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn refresh_line(&self, lines: Vec<u16>) {
        let stdout = &mut io::stdout();
        for row_index in lines {
            execute!(stdout, crossterm::cursor::MoveTo(0, row_index)).unwrap();
            let row_data = self.game_board[row_index as usize];
            for content in row_data {
                match content {
                    None => {
                        execute!(stdout, crossterm::style::SetBackgroundColor(Color::White))
                            .unwrap();
                        write!(io::stdout(), "  ").unwrap();
                    }
                    Some(color) => {
                        execute!(stdout, crossterm::style::SetBackgroundColor(color)).unwrap();
                        write!(io::stdout(), "  ").unwrap();
                    }
                }
            }
            stdout.flush().unwrap();
        }
    }
}
