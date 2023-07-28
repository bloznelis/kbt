use std::sync::{mpsc::Sender, Arc, Mutex};

use device_query_revamped::{CallbackGuard, DeviceEvents, DeviceState, Keycode};

use crate::{
    key::Key,
    model::{AppEvent, KeyEventType},
};

pub struct GenericKeyBackend;

type KeycodeCallback = Box<dyn Fn(&Keycode) -> () + Send + Sync + 'static>;
type KeyStreamGuard = CallbackGuard<KeycodeCallback>;

impl GenericKeyBackend {
    pub fn subscribe(sender: &Sender<AppEvent>) -> (KeyStreamGuard, KeyStreamGuard) {
        let device_state = DeviceState::new();
        let shared_sender = Arc::new(Mutex::new(sender.clone()));

        let key_up_guard: KeyStreamGuard = device_state.on_key_up(Box::new(move |keycode| {
            let _ = shared_sender
                .lock()
                .unwrap()
                .send(AppEvent::KeyEvent(KeyEventType::KeyReleased(map_keycode(
                    keycode,
                ))))
                .map_err(|err| log::error!("Key down channel died {}", err));
        }));

        let shared_sender = Arc::new(Mutex::new(sender.clone()));

        let key_down_guard: KeyStreamGuard = device_state.on_key_down(Box::new(move |keycode| {
            let _ = shared_sender
                .lock()
                .unwrap()
                .send(AppEvent::KeyEvent(KeyEventType::KeyPressed(map_keycode(
                    keycode,
                ))))
                .map_err(|err| log::error!("Key down channel died {}", err));
        }));

        (key_up_guard, key_down_guard)
    }
}

fn map_keycode(keycode: &Keycode) -> Key {
    match keycode {
        Keycode::Key0 => Key::Zero,
        Keycode::Key1 => Key::One,
        Keycode::Key2 => Key::Two,
        Keycode::Key3 => Key::Three,
        Keycode::Key4 => Key::Four,
        Keycode::Key5 => Key::Five,
        Keycode::Key6 => Key::Six,
        Keycode::Key7 => Key::Seven,
        Keycode::Key8 => Key::Eight,
        Keycode::Key9 => Key::Nine,
        Keycode::A => Key::A,
        Keycode::B => Key::B,
        Keycode::C => Key::C,
        Keycode::D => Key::D,
        Keycode::E => Key::E,
        Keycode::F => Key::F,
        Keycode::G => Key::G,
        Keycode::H => Key::H,
        Keycode::I => Key::I,
        Keycode::J => Key::J,
        Keycode::K => Key::K,
        Keycode::L => Key::L,
        Keycode::M => Key::M,
        Keycode::N => Key::N,
        Keycode::O => Key::O,
        Keycode::P => Key::P,
        Keycode::Q => Key::Q,
        Keycode::R => Key::R,
        Keycode::S => Key::S,
        Keycode::T => Key::T,
        Keycode::U => Key::U,
        Keycode::V => Key::V,
        Keycode::W => Key::W,
        Keycode::X => Key::X,
        Keycode::Y => Key::Y,
        Keycode::Z => Key::Z,
        Keycode::F1 => Key::F1,
        Keycode::F2 => Key::F2,
        Keycode::F3 => Key::F3,
        Keycode::F4 => Key::F4,
        Keycode::F5 => Key::F5,
        Keycode::F6 => Key::F6,
        Keycode::F7 => Key::F7,
        Keycode::F8 => Key::F8,
        Keycode::F9 => Key::F9,
        Keycode::F10 => Key::F10,
        Keycode::F11 => Key::F11,
        Keycode::F12 => Key::F12,
        Keycode::Escape => Key::Esc,
        Keycode::Space => Key::Spacebar,
        Keycode::LControl => Key::LeftCtrl,
        Keycode::RControl => Key::RightCtrl,
        Keycode::LShift => Key::LeftShift,
        Keycode::RShift => Key::RightShift,
        Keycode::LAlt => Key::LeftAlt,
        Keycode::RAlt => Key::RightAlt,
        Keycode::Command => Key::Command,
        Keycode::LMeta => Key::LeftSuper,
        Keycode::RMeta => Key::RightSuper,
        Keycode::Enter => Key::Return,
        Keycode::Up => Key::ArrowUp,
        Keycode::Down => Key::ArrowDown,
        Keycode::Left => Key::ArrowLeft,
        Keycode::Right => Key::ArrowRight,
        Keycode::Backspace => Key::Backspace,
        Keycode::CapsLock => Key::CapsLock,
        Keycode::Tab => Key::Tab,
        Keycode::Home => Key::Home,
        Keycode::End => Key::End,
        Keycode::PageUp => Key::PgUp,
        Keycode::PageDown => Key::PgDown,
        Keycode::Insert => Key::Insert,
        Keycode::Delete => Key::Delete,
        Keycode::Grave => Key::Grave,
        Keycode::Minus => Key::Hyphen,
        Keycode::Equal => Key::Plus,
        Keycode::LeftBracket => Key::LeftBracket,
        Keycode::RightBracket => Key::RightBracket,
        Keycode::BackSlash => Key::Backslash,
        Keycode::Semicolon => Key::SemiColon,
        Keycode::Apostrophe => Key::Apostrophe,
        Keycode::Comma => Key::Comma,
        Keycode::Dot => Key::Period,
        Keycode::Slash => Key::QuestionMark,
        _ => Key::Unknown,
    }
}
