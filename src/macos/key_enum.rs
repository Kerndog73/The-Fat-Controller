// This file was generated by build.rs

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Key {
    A = 0,
    S = 1,
    D = 2,
    F = 3,
    H = 4,
    G = 5,
    Z = 6,
    X = 7,
    C = 8,
    V = 9,
    B = 11,
    Q = 12,
    W = 13,
    E = 14,
    R = 15,
    Y = 16,
    T = 17,
    N1 = 18,
    N2 = 19,
    N3 = 20,
    N4 = 21,
    N6 = 22,
    N5 = 23,
    Equal = 24,
    N9 = 25,
    N7 = 26,
    Minus = 27,
    N8 = 28,
    N0 = 29,
    RightBracket = 30,
    O = 31,
    U = 32,
    LeftBracket = 33,
    I = 34,
    P = 35,
    L = 37,
    J = 38,
    Quote = 39,
    K = 40,
    Semicolon = 41,
    Backslash = 42,
    Comma = 43,
    Slash = 44,
    N = 45,
    M = 46,
    Period = 47,
    Grave = 50,
    KeypadDecimal = 65,
    KeypadMultiply = 67,
    KeypadPlus = 69,
    KeypadClear = 71,
    KeypadDivide = 75,
    KeypadEnter = 76,
    KeypadMinus = 78,
    KeypadEquals = 81,
    Keypad0 = 82,
    Keypad1 = 83,
    Keypad2 = 84,
    Keypad3 = 85,
    Keypad4 = 86,
    Keypad5 = 87,
    Keypad6 = 88,
    Keypad7 = 89,
    Keypad8 = 91,
    Keypad9 = 92,
    Return = 36,
    Tab = 48,
    Space = 49,
    Delete = 51,
    Escape = 53,
    Command = 55,
    Shift = 56,
    CapsLock = 57,
    Option = 58,
    Control = 59,
    RightCommand = 54,
    RightShift = 60,
    RightOption = 61,
    RightControl = 62,
    Function = 63,
    F17 = 64,
    VolumeUp = 72,
    VolumeDown = 73,
    Mute = 74,
    F18 = 79,
    F19 = 80,
    F20 = 90,
    F5 = 96,
    F6 = 97,
    F7 = 98,
    F3 = 99,
    F8 = 100,
    F9 = 101,
    F11 = 103,
    F13 = 105,
    F16 = 106,
    F14 = 107,
    F10 = 109,
    F12 = 111,
    F15 = 113,
    Help = 114,
    Home = 115,
    PageUp = 116,
    ForwardDelete = 117,
    F4 = 118,
    End = 119,
    F2 = 120,
    PageDown = 121,
    F1 = 122,
    LeftArrow = 123,
    RightArrow = 124,
    DownArrow = 125,
    UpArrow = 126,
}

impl std::str::FromStr for Key {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Key::A),
            "s" => Ok(Key::S),
            "d" => Ok(Key::D),
            "f" => Ok(Key::F),
            "h" => Ok(Key::H),
            "g" => Ok(Key::G),
            "z" => Ok(Key::Z),
            "x" => Ok(Key::X),
            "c" => Ok(Key::C),
            "v" => Ok(Key::V),
            "b" => Ok(Key::B),
            "q" => Ok(Key::Q),
            "w" => Ok(Key::W),
            "e" => Ok(Key::E),
            "r" => Ok(Key::R),
            "y" => Ok(Key::Y),
            "t" => Ok(Key::T),
            "1" => Ok(Key::N1),
            "2" => Ok(Key::N2),
            "3" => Ok(Key::N3),
            "4" => Ok(Key::N4),
            "6" => Ok(Key::N6),
            "5" => Ok(Key::N5),
            "equal" => Ok(Key::Equal),
            "9" => Ok(Key::N9),
            "7" => Ok(Key::N7),
            "minus" => Ok(Key::Minus),
            "8" => Ok(Key::N8),
            "0" => Ok(Key::N0),
            "rightbracket" => Ok(Key::RightBracket),
            "o" => Ok(Key::O),
            "u" => Ok(Key::U),
            "leftbracket" => Ok(Key::LeftBracket),
            "i" => Ok(Key::I),
            "p" => Ok(Key::P),
            "l" => Ok(Key::L),
            "j" => Ok(Key::J),
            "quote" => Ok(Key::Quote),
            "k" => Ok(Key::K),
            "semicolon" => Ok(Key::Semicolon),
            "backslash" => Ok(Key::Backslash),
            "comma" => Ok(Key::Comma),
            "slash" => Ok(Key::Slash),
            "n" => Ok(Key::N),
            "m" => Ok(Key::M),
            "period" => Ok(Key::Period),
            "grave" => Ok(Key::Grave),
            "keypaddecimal" => Ok(Key::KeypadDecimal),
            "keypadmultiply" => Ok(Key::KeypadMultiply),
            "keypadplus" => Ok(Key::KeypadPlus),
            "keypadclear" => Ok(Key::KeypadClear),
            "keypaddivide" => Ok(Key::KeypadDivide),
            "keypadenter" => Ok(Key::KeypadEnter),
            "keypadminus" => Ok(Key::KeypadMinus),
            "keypadequals" => Ok(Key::KeypadEquals),
            "keypad0" => Ok(Key::Keypad0),
            "keypad1" => Ok(Key::Keypad1),
            "keypad2" => Ok(Key::Keypad2),
            "keypad3" => Ok(Key::Keypad3),
            "keypad4" => Ok(Key::Keypad4),
            "keypad5" => Ok(Key::Keypad5),
            "keypad6" => Ok(Key::Keypad6),
            "keypad7" => Ok(Key::Keypad7),
            "keypad8" => Ok(Key::Keypad8),
            "keypad9" => Ok(Key::Keypad9),
            "return" => Ok(Key::Return),
            "tab" => Ok(Key::Tab),
            "space" => Ok(Key::Space),
            "delete" => Ok(Key::Delete),
            "escape" => Ok(Key::Escape),
            "command" => Ok(Key::Command),
            "shift" => Ok(Key::Shift),
            "capslock" => Ok(Key::CapsLock),
            "option" => Ok(Key::Option),
            "control" => Ok(Key::Control),
            "rightcommand" => Ok(Key::RightCommand),
            "rightshift" => Ok(Key::RightShift),
            "rightoption" => Ok(Key::RightOption),
            "rightcontrol" => Ok(Key::RightControl),
            "function" => Ok(Key::Function),
            "f17" => Ok(Key::F17),
            "volumeup" => Ok(Key::VolumeUp),
            "volumedown" => Ok(Key::VolumeDown),
            "mute" => Ok(Key::Mute),
            "f18" => Ok(Key::F18),
            "f19" => Ok(Key::F19),
            "f20" => Ok(Key::F20),
            "f5" => Ok(Key::F5),
            "f6" => Ok(Key::F6),
            "f7" => Ok(Key::F7),
            "f3" => Ok(Key::F3),
            "f8" => Ok(Key::F8),
            "f9" => Ok(Key::F9),
            "f11" => Ok(Key::F11),
            "f13" => Ok(Key::F13),
            "f16" => Ok(Key::F16),
            "f14" => Ok(Key::F14),
            "f10" => Ok(Key::F10),
            "f12" => Ok(Key::F12),
            "f15" => Ok(Key::F15),
            "help" => Ok(Key::Help),
            "home" => Ok(Key::Home),
            "pageup" => Ok(Key::PageUp),
            "forwarddelete" => Ok(Key::ForwardDelete),
            "f4" => Ok(Key::F4),
            "end" => Ok(Key::End),
            "f2" => Ok(Key::F2),
            "pagedown" => Ok(Key::PageDown),
            "f1" => Ok(Key::F1),
            "leftarrow" => Ok(Key::LeftArrow),
            "rightarrow" => Ok(Key::RightArrow),
            "downarrow" => Ok(Key::DownArrow),
            "uparrow" => Ok(Key::UpArrow),
            _ => Err(()),
        }
    }
}
