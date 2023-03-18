use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::{AppEvent, Key};
use crate::{KeyBackend, KeyEventType};
use x11rb::connection::Connection;
use x11rb::protocol::{xproto::*, Event};
use x11rb::rust_connection::RustConnection;

pub struct X11;

impl KeyBackend for X11 {
    fn subscribe(&self, sender: Sender<AppEvent>) -> Result<(), Box<dyn std::error::Error>> {
        thread::spawn(move || listen_for_keypresses(sender).unwrap());

        Ok(())
    }
}

fn listen_for_keypresses(sender: Sender<AppEvent>) -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];
    let net_active_window: Window = get_or_intern_atom(&conn, b"_NET_ACTIVE_WINDOW");
    let focus = find_active_window(&conn, screen.root, net_active_window).unwrap();

    conn.change_window_attributes(
        focus,
        &ChangeWindowAttributesAux::new().event_mask(EventMask::KEY_PRESS | EventMask::KEY_RELEASE),
    )?;

    conn.flush()?;

    loop {
        let event = conn.wait_for_event()?;
        match event {
            Event::KeyPress(key_press) => {
                let key: Key = key_press.detail.into();

                sender.send(AppEvent::KeyEvent(KeyEventType::KeyPressed(key)))?;
            }
            Event::KeyRelease(key_release) => {
                let key: Key = key_release.detail.into();
                sender.send(AppEvent::KeyEvent(KeyEventType::KeyReleased(key)))?;
            }
            // _ => println!("unexpected event! {:?}", event),
            _ => (),
        }
    }
}

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

impl From<u8> for Key {
    fn from(value: u8) -> Self {
        match value {
            23 => Key::Tab,
            24 => Key::Q,
            25 => Key::W,
            26 => Key::E,
            27 => Key::R,
            28 => Key::T,
            29 => Key::Y,
            30 => Key::U,
            31 => Key::I,
            32 => Key::O,
            33 => Key::P,
            34 => Key::LeftBracket,
            35 => Key::RightBracket,
            36 => Key::Return,
            37 => Key::LeftCtrl,
            38 => Key::A,
            39 => Key::S,
            40 => Key::D,
            41 => Key::F,
            42 => Key::G,
            43 => Key::H,
            44 => Key::J,
            45 => Key::K,
            46 => Key::L,
            50 => Key::LeftShift,
            51 => Key::Backslash,
            52 => Key::Z,
            53 => Key::X,
            54 => Key::C,
            55 => Key::V,
            56 => Key::B,
            57 => Key::N,
            58 => Key::M,
            62 => Key::RightShift,
            _ => Key::Unknown,
        }
    }
}
