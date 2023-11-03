use crossterm::style::Color;

static RED_CONTENT: [[[u8; 4]; 4]; 4] = [
    [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 1, 1], [0, 0, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
    [[0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
    [[0, 0, 1, 0], [0, 0, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
];

static BLUE_CONTENT: [[[u8; 4]; 4]; 4] = [
    [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0]],
    [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
    [[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
    [[0, 0, 0, 0], [0, 0, 1, 1], [0, 1, 1, 0], [0, 0, 0, 0]],
];

static YELLOW_CONTENT: [[[u8; 4]; 4]; 4] = [
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
];

static GREEN_CONTENT: [[[u8; 4]; 4]; 4] = [
    [[0, 0, 0, 0], [0, 0, 1, 0], [0, 1, 1, 1], [0, 0, 0, 0]],
    [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0]],
    [[0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
    [[0, 0, 0, 0], [0, 0, 1, 0], [0, 1, 1, 0], [0, 0, 1, 0]],
];

static CYAN_CONTENT: [[[u8; 4]; 4]; 4] = [
    [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
    [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
    [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
    [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
];

#[derive(Debug)]
pub(crate) struct Tiles {
    color: Color,
    pub(crate) content: [[[u8; 4]; 4]; 4],
}

impl Tiles {
    fn new(color: Color, content: [[[u8; 4]; 4]; 4]) -> Tiles {
        Tiles { color, content }
    }
}

pub(crate) fn get_tiles_vec() -> Vec<Tiles> {
    vec![
        Tiles::new(Color::Red, RED_CONTENT),
        Tiles::new(Color::Blue, BLUE_CONTENT),
        Tiles::new(Color::Yellow, YELLOW_CONTENT),
        Tiles::new(Color::Green, GREEN_CONTENT),
        Tiles::new(Color::Cyan, CYAN_CONTENT),
    ]
}
