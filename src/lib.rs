use std::io;
use std::io::Write;
use clap::Parser;
use crossterm::execute;
use crossterm::terminal::DisableLineWrap;

// cargo run -- -h
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arg {
    // development mode
    #[arg(short, long)]
    dev: bool,

}

pub fn run() {
    let arg = Arg::parse();
    println!("{:?}", arg);
    execute!(io::stdout(), DisableLineWrap, crossterm::terminal::SetSize(20, 20)).unwrap();
    // 将从 0:0 开始到 20:20 的区域填充为白色
    execute!(io::stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    // 隐藏光标
    execute!(io::stdout(), crossterm::cursor::Hide).unwrap();
    // 输入背景色
    for index in 0..20 {
        execute!(io::stdout(), crossterm::style::SetBackgroundColor(crossterm::style::Color::White)).unwrap();
        execute!(
            io::stdout(),
            crossterm::cursor::MoveTo(0, index),
            Write
        ).unwrap();
        write!("                    ", "");
        // 打印空字符串
        io::stdout().flush().unwrap();
        execute!(io::stdout(), crossterm::style::SetBackgroundColor(crossterm::style::Color::Black)).unwrap();
    }
    loop {
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
}