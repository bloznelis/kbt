use crate::{key::Key, KeyUI, KeySize};

pub const R4_1: [KeyUI; 14] = [
    KeyUI {
        key: Key::Grave,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::One,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Two,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Three,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Four,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Five,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Six,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Seven,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Eight,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Nine,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Zero,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Hyphen,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Plus,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Backspace,
        size: KeySize::U2,
        size_correction: None,
    },
];

pub const R3: [KeyUI; 14] = [
    KeyUI {
        key: Key::Tab,
        size: KeySize::U15,
        size_correction: None,
    },
    KeyUI {
        key: Key::Q,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::W,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::E,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::R,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::T,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Y,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::U,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::I,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::O,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::P,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::LeftBracket,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::RightBracket,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Backslash,
        size: KeySize::U15,
        size_correction: Some(1),
    },
];

pub const R2: [KeyUI; 13] = [
    KeyUI {
        key: Key::CapsLock,
        size: KeySize::U175,
        size_correction: None,
    },
    KeyUI {
        key: Key::A,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::S,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::D,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::F,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::G,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::H,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::J,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::K,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::L,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::SemiColon,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Apostrophe,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Return,
        size: KeySize::U250,
        size_correction: None,
    },
];

pub const R1_0: [KeyUI; 12] = [
    KeyUI {
        key: Key::LeftShift,
        size: KeySize::U225,
        size_correction: None,
    },
    KeyUI {
        key: Key::Z,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::X,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::C,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::V,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::B,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::N,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::M,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Comma,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::Period,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::QuestionMark,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::RightShift,
        size: KeySize::U275,
        size_correction: None,
    },
];

pub const R1_1: [KeyUI; 7] = [
    KeyUI {
        key: Key::LeftCtrl,
        size: KeySize::U15,
        size_correction: None,
    },
    KeyUI {
        key: Key::LeftSuper,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::LeftAlt,
        size: KeySize::U15,
        size_correction: None,
    },
    KeyUI {
        key: Key::Spacebar,
        size: KeySize::U675,
        size_correction: Some(3),
    },
    KeyUI {
        key: Key::RightAlt,
        size: KeySize::U15,
        size_correction: None,
    },
    KeyUI {
        key: Key::RightSuper,
        size: KeySize::U1,
        size_correction: None,
    },
    KeyUI {
        key: Key::RightCtrl,
        size: KeySize::U15,
        size_correction: None,
    },
];
