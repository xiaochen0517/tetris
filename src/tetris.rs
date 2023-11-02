use crossterm::style::Color;

#[derive(Debug)]
struct Tiles {
    color: Color,
    content: [[u8; 4]; 2],
}

impl Tiles {
    fn new(color: Color, content: [[u8; 4]; 2]) -> Tiles {
        Tiles {
            color,
            content,
        }
    }
}

#[derive(Debug)]
struct Canvas {
    game_board: [[u8; 20]; 20],
    score: u32,
    tiles_vec: Vec<Tiles>,
}

impl Canvas {
    fn new() -> Canvas {
        let red_tiles = Tiles::new(Color::Red, [[1, 0, 0, 0], [1, 1, 1, 0]]);
        let blue_tiles = Tiles::new(Color::Blue, [[1, 1, 1, 1], [0, 0, 0, 0]]);
        let yellow_tiles = Tiles::new(Color::Yellow, [[1, 1, 0, 0], [1, 1, 0, 0]]);
        let green_tiles = Tiles::new(Color::Green, [[0, 1, 0, 0], [1, 1, 1, 0]]);
        let cyan_tiles = Tiles::new(Color::Cyan, [[1, 1, 0, 0], [0, 1, 1, 0]]);
        Canvas {
            game_board: [[0; 20]; 20],
            score: 0,
            tiles_vec: vec![
                red_tiles,
                blue_tiles,
                yellow_tiles,
                green_tiles,
                cyan_tiles,
            ],
        }
    }
}

