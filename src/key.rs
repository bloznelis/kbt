use std::fmt;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Key {
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    // Symbols
    LeftBracket,
    RightBracket,
    Backslash,
    // modifiers
    Tab,
    CapsLock,
    LeftShift,
    LeftCtrl,
    LeftSuper,
    LeftAlt,
    Spacebar,
    RightAlt,
    RightSuper,
    RightCtrl,
    RightShift,
    Return,
    Backspace,

    Unknown,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Key::LeftBracket => write!(f, "[{{"),
            Key::RightBracket => write!(f, "]}}"),
            Key::Backslash => write!(f, "|\\"),
            _ => write!(f, "{:?}", self),
        }
    }
}
