use std::sync::mpsc::Sender;
use std::thread;

use crate::{AppEvent, KbtError, Key};
use crate::{KeyBackend, KeyEventType};
use x11rb::connection::Connection;
use x11rb::protocol::{xproto::*, Event};
use x11rb::rust_connection::RustConnection;

pub struct X11;

impl KeyBackend for X11 {
    fn subscribe(&self, sender: Sender<AppEvent>) -> Result<(), KbtError> {
        thread::spawn(move || {
            listen_for_keypresses(sender).map_err(|err| KbtError {
                message: format!("keypress listener fork died msg={}", err.to_string()),
            })
        });

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
            9 => Key::Esc,
            10 => Key::One,
            11 => Key::Two,
            12 => Key::Three,
            13 => Key::Four,
            14 => Key::Five,
            15 => Key::Six,
            16 => Key::Seven,
            17 => Key::Eight,
            18 => Key::Nine,
            19 => Key::Zero,
            20 => Key::Hyphen,
            21 => Key::Plus,
            22 => Key::Backspace,
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
            47 => Key::SemiColon,
            48 => Key::Apostrophe,
            49 => Key::Grave,
            50 => Key::LeftShift,
            51 => Key::Backslash,
            52 => Key::Z,
            53 => Key::X,
            54 => Key::C,
            55 => Key::V,
            56 => Key::B,
            57 => Key::N,
            58 => Key::M,
            59 => Key::Comma,
            60 => Key::Period,
            61 => Key::QuestionMark,
            62 => Key::RightShift,
            64 => Key::LeftAlt,
            65 => Key::Spacebar,
            66 => Key::CapsLock,
            67 => Key::F1,
            68 => Key::F2,
            69 => Key::F3,
            70 => Key::F4,
            71 => Key::F5,
            72 => Key::F6,
            73 => Key::F7,
            74 => Key::F8,
            75 => Key::F9,
            76 => Key::F10,
            78 => Key::ScrollLock,

            95 => Key::F11,
            96 => Key::F12,
            105 => Key::RightCtrl,
            108 => Key::RightAlt,
            107 => Key::PrintScreen,
            110 => Key::Home,
            111 => Key::ArrowUp,
            112 => Key::PgUp,
            113 => Key::ArrowLeft,
            114 => Key::ArrowRight,
            115 => Key::End,
            116 => Key::ArrowDown,
            117 => Key::PgDown,
            118 => Key::Insert,
            119 => Key::Delete,
            127 => Key::PauseBreak,
            133 => Key::LeftSuper,
            134 => Key::RightSuper,
            _ => Key::Unknown,
        }
    }
}
