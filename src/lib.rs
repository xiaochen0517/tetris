mod tetris;
mod tiles;

use clap::Parser;
use crossterm::style::Color;
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute};
use std::io;
use std::io::Write;

// cargo run -- -h
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arg {
    // development mode
    #[arg(short, long)]
    dev: bool,
}

pub fn run() {
    // 进入raw模式
    crossterm::terminal::enable_raw_mode().unwrap();
    // 进入备用屏幕
    execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
    let arg = Arg::parse();
    println!("{:?}", arg);
    // 将从 0:0 开始到 20:20 的区域填充为白色
    execute!(io::stdout(), crossterm::terminal::Clear(ClearType::All)).unwrap();
    // 隐藏光标
    execute!(io::stdout(), cursor::Hide).unwrap();
    // 输入背景色
    for index in 0..20 {
        execute!(
            io::stdout(),
            crossterm::cursor::MoveTo(0, index),
            crossterm::style::SetBackgroundColor(Color::White)
        )
        .unwrap();
        write!(io::stdout(), "　　　　　　　　　　").unwrap();
        execute!(
            io::stdout(),
            crossterm::style::SetBackgroundColor(Color::Black)
        )
        .unwrap();
        match index {
            1 => {
                write!(io::stdout(), "      Score: 0      ").unwrap();
            }
            2 => {
                write!(io::stdout(), "      Level: 1      ").unwrap();
            }
            _ => {
                write!(io::stdout(), "　　　　　　　　　　").unwrap();
            }
        }
        write!(io::stdout(), "|").unwrap();
        // 打印空字符串
    }
    io::stdout().flush().unwrap();
    execute!(io::stdout(), crossterm::cursor::MoveTo(0, 20)).unwrap();
    write!(io::stdout(), "————————————————————————————————————————").unwrap();
    execute!(io::stdout(), crossterm::cursor::MoveTo(0, 21)).unwrap();
    write!(io::stdout(), "Press Q to exit.").unwrap();
    io::stdout().flush().unwrap();
    let mut canvas = tetris::Canvas::new();
    canvas.new_tail(0, 0);
    canvas.tile_position.1 = 0;
    canvas.draw_tail(tetris::Direction::Down);
    // game loop
    loop {
        // 等待 16ms
        std::thread::sleep(std::time::Duration::from_millis(1000));
        // 向下移动tail
        canvas.tile_position.1 += 1;
        canvas.draw_tail(tetris::Direction::Down);
        // 等待输入q
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key) => {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
            _ => {}
        }
    }
    // 显示光标
    execute!(io::stdout(), cursor::Show).unwrap();
    // 退出备用屏幕
    execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();
    // 退出raw模式
    crossterm::terminal::disable_raw_mode().unwrap();
}
