use std::cmp;

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
    fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        let x = (self.x + dx) as usize;
        let y = (self.y + dy) as usize;

        if !game.map[x][y].blocked {
            self.x += dx;
            self.y += dy;
        }
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
#[derive(Clone)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

/// A rectangle on the map. Used to characterize a room.
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }
}

type Map = Vec<Vec<Tile>>;
struct Game {
    map: Map,
}

fn make_map() -> Map {
    // Fill map with "blocked" tiles
    let mut map = vec![
        vec![Tile{ blocked: true, block_sight: true }; MAP_HEIGHT as usize];
        MAP_WIDTH as usize
    ];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(50, 15, 10, 15);

    create_room(&mut map, room1);
    create_room(&mut map, room2);

    create_h_tunnel(&mut map, 25, 55, 23);

    map
}

/// Go through the tiles in the rectangle and make them passable
fn create_room(map: &mut Map, room: Rect) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize].blocked = false;
            map[x as usize][y as usize].block_sight = false;
        }
    }
}

/// Create horizontal tunnel
fn create_h_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
    for x in cmp::min(x1, x2)..cmp::max(x1, x2) {
        map[x as usize][y as usize].blocked = false;
        map[x as usize][y as usize].block_sight = false;
    }
}

/// Create vertical tunnel
fn create_v_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
    for y in cmp::min(y1, y2)..cmp::max(y1, y2) {
        map[x as usize][y as usize].blocked = false;
        map[x as usize][y as usize].block_sight = false;
    }
}

/// Draw all objects in the list
fn render_all(tcod: &mut Tcod, map: &mut Map, objects: &[Object; 2]) {
    for object in objects {
        object.draw(&mut tcod.con);
    }

    map[30][22].blocked = true;
    map[30][22].block_sight = true;
    map[50][22].blocked = true;
    map[50][22].block_sight = true;

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = map[x as usize][y as usize].block_sight;

            match wall {
                true => tcod.con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set),
                false => tcod.con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set),
            }
        }
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
}

/// Handle key events from user
fn handle_keys(object: &mut Object, game: &Game, key_code: KeyCode) -> bool {
    match key_code {
        KeyCode::Up => object.move_by(0, -1, game),
        KeyCode::Down => object.move_by(0, 1, game),
        KeyCode::Left => object.move_by(-1, 0, game),
        KeyCode::Right => object.move_by(1, 0, game),

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

    let mut game = Game {
        map: make_map()
    };

    let player = &mut objects[0];
    player.x = 25;
    player.y = 23;

    while !tcod.root.window_closed() {
        tcod.con.clear();

        render_all(&mut tcod, &mut game.map, &objects);

        tcod.root.flush();

        let key = tcod.root.wait_for_keypress(true);

        if key.pressed {
            let player = &mut objects[0];
            let exit = handle_keys(player, &game, key.code);
            if exit {
                break;
            }
        }
    }
}
