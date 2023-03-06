use tcod::system;
use tcod::colors::WHITE;
use tcod::console::*;
use tcod::input::KeyCode;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
}

struct Player {
    x: i32,
    y: i32,
}

fn handle_keys(player: &mut Player, key_code: KeyCode) -> bool {
    match key_code {
        KeyCode::Up => player.y -= 1,
        KeyCode::Down => player.y += 1,
        KeyCode::Left => player.x -= 1,
        KeyCode::Right => player.x += 1,

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

    let mut tcod = Tcod { root };

    let mut player = Player {
        x: SCREEN_WIDTH / 2,
        y: SCREEN_HEIGHT / 2,
    };

    while !tcod.root.window_closed() {
        tcod.root.clear();
        tcod.root.set_default_foreground(WHITE);
        tcod.root.put_char(player.x, player.y, '@', BackgroundFlag::None);
        tcod.root.flush();

        let key = tcod.root.wait_for_keypress(true);

        if key.pressed {
            let exit = handle_keys(&mut player, key.code);
            if exit {
                break;
            }
        }
    }
}
