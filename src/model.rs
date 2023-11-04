use std::{collections::HashMap, fmt::Display, sync::mpsc::Receiver};

use crate::key::Key;

pub struct App {
    pub key_states: HashMap<Key, KeyState>,
    pub event_receiver: Receiver<AppEvent>,
    pub keyboard_size: KeyboardSize,
}

impl App {
    pub fn reset(&mut self) {
        self.key_states = HashMap::new()
    }
}

pub struct KeyUI {
    pub key: Key,
    pub size: KeySize,
    pub size_correction: Option<i16>, // To make layout look consistent
}

pub enum KeySize {
    U05,
    U1,
    U15,
    U175,
    U2,
    U225,
    U250,
    U275,
    U4,
    U675,
}

// 1 terminal cell = 0.2u

//  1u (len = 5)
// ┏━━━┓
// ┃ A ┃
// ┗━━━┛

//  2u (len = 10)
// ┏━━━━━━━━┓
// ┃  |<-   ┃
// ┗━━━━━━━━┛
impl KeySize {
    pub fn static_len(&self) -> i16 {
        match self {
            KeySize::U05 => 2,
            KeySize::U1 => 5,
            KeySize::U15 => 7,
            KeySize::U175 => 8,
            KeySize::U2 => 10,
            KeySize::U225 => 11,
            KeySize::U250 => 12,
            KeySize::U275 => 14,
            KeySize::U4 => 20,
            KeySize::U675 => 34,
        }
    }
}

pub enum MenuResult {
    KeyboardSelected(KeyboardSize),
    Terminate,
}

pub enum KeyState {
    Pressed,
    Released,
    Untouched,
}

pub enum KeyEventType {
    KeyPressed(Key),
    KeyReleased(Key),
}

pub enum ControlEventType {
    Terminate,
    Reset,
}

pub enum AppEvent {
    KeyEvent(KeyEventType),
    ControlEvent(ControlEventType),
    ScreenResize,
}

#[derive(Clone)]
pub enum KeyboardSize {
    Keyboard60,
    Keyboard80,
    Keyboard100,
}

impl Display for KeyboardSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyboardSize::Keyboard60 => write!(f, "60% layout"),
            KeyboardSize::Keyboard80 => write!(f, "80% layout"),
            KeyboardSize::Keyboard100 => write!(f, "100% layout"),
        }
    }
}

#[derive(Debug)]
pub struct KbtError {
    pub message: String,
}

impl<T: ToString> From<T> for KbtError {
    fn from(value: T) -> Self {
        KbtError {
            message: value.to_string(),
        }
    }
}
