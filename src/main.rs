use tcod::system;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::KeyCode;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;
const COLOR_DARK_WALL: Color = Color{ r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color{ r: 50, g: 50, b: 150 };

struct Tcod {
    root: Root,
    con: Offscreen,
}

/// This is a generic object: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen.
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    /// Move by the given amount
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    /// Set the color and then draw the character that represents this object at its position
    fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    /// Erase the character that represents this object
    fn clear(&self, con: &mut dyn Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

/// A tile of the map and its properties
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    fn new(blocked: bool, block_sight: Option<bool>) -> Tile {
        // By default, if a tile is blocked, it also blocks sight
        let mut sight_blocked = block_sight.is_some();

        if block_sight.is_none() {
            sight_blocked = blocked;
        }

        Tile { blocked, block_sight: sight_blocked }
    }
}

fn handle_keys(object: &mut Object, key_code: KeyCode) -> bool {
    match key_code {
        KeyCode::Up => object.move_by(0, -1),
        KeyCode::Down => object.move_by(0, 1),
        KeyCode::Left => object.move_by(-1, 0),
        KeyCode::Right => object.move_by(1, 0),

        KeyCode::Escape => return true,

        _ => ()
    };

    false
}

fn main() {
    system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Roguelike")
        .fullscreen(false)
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, con };

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);
    let mut objects = [player, npc];

    while !tcod.root.window_closed() {
        tcod.con.clear();

        for object in &objects {
            object.draw(&mut tcod.con);
        }

        blit(
            &tcod.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0
        );

        tcod.root.flush();

        let key = tcod.root.wait_for_keypress(true);

        if key.pressed {
            let player = &mut objects[0];
            let exit = handle_keys(player, key.code);
            if exit {
                break;
            }
        }
    }
}
