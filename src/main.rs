use x11rb::connection::Connection;
use x11rb::errors::ReplyOrIdError;
use x11rb::protocol::{xproto::*, Event};
use x11rb::rust_connection::RustConnection;
use x11rb::COPY_DEPTH_FROM_PARENT;

fn get_or_intern_atom(conn: &RustConnection, name: &[u8]) -> Atom {
    let result = conn
        .intern_atom(false, name)
        .expect("Failed to intern atom")
        .reply()
        .expect("Failed receive interned atom");

    result.atom
}

fn find_active_window(
    conn: &impl Connection,
    root: Window,
    net_active_window: Atom,
) -> Option<Window> {
    let window: Atom = AtomEnum::WINDOW.into();
    let active_window = conn
        .get_property(false, root, net_active_window, window, 0, 1)
        .expect("Failed to get X11 property")
        .reply()
        .expect("Failed to receive X11 property reply");

    if active_window.format == 32 && active_window.length == 1 {
        active_window
            .value32()
            .expect("Invalid message. Expected value with format = 32")
            .next()
    } else {
        // Query the input focus
        Some(
            conn.get_input_focus()
                .expect("Failed to get input focus")
                .reply()
                .expect("Failed to receive X11 input focus")
                .focus,
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];
    let win_id = conn.generate_id()?;
    let net_active_window: Window = get_or_intern_atom(&conn, b"_NET_ACTIVE_WINDOW");
    let focus = find_active_window(&conn, screen.root, net_active_window).unwrap();

    // Emit focused window key presses
    conn.change_window_attributes(
        focus,
        &ChangeWindowAttributesAux::new().event_mask(EventMask::KEY_PRESS),
    )?;
    conn.flush();
    loop {
        let event = conn.wait_for_event().unwrap();
        match event {
            Event::KeyPress(key_press) => println!("  pressed {}", key_press.detail),
            // Event::KeyRelease(key_release) => println!("released {}", key_release.detail),
            // _ => println!("unexpected event! {:?}", event),
            _ => (),
        }
    }
}
