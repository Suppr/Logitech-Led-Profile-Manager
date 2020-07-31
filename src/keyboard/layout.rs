use std::collections::hash_map::HashMap;
use serde::{Deserialize, Serialize};
pub type KeysDB = HashMap<Key, klogi::model::Keydata>;
pub type KeydataType = (Key, klogi::model::Keydata);

//keyboard bit map representation
#[repr(C)]
#[derive(Clone, Serialize, Deserialize, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types, non_snake_case)]
pub enum Key {
    ESC = 0,
    F1 = 1,
    F2 = 2,
    F3 = 3,
    F4 = 4,
    F5 = 5,
    F6 = 6,
    F7 = 7,
    F8 = 8,
    F9 = 9,
    F10 = 10,
    F11 = 11,
    F12 = 12,
    PRINT_SCREEN = 13,
    SCROLL_LOCK = 14,
    PAUSE_BREAK = 15,

    TILDE = 21,
    ONE = 22,
    TWO = 23,
    THREE = 24,
    FOUR = 25,
    FIVE = 26,
    SIX = 27,
    SEVEN = 28,
    EIGHT = 29,
    NINE = 30,
    ZERO = 31,
    MINUS = 32,
    EQUALS = 33,
    BACKSPACE = 34,
    INSERT = 35,
    HOME = 36,
    PAGE_UP = 37,
    NUM_LOCK = 38,
    NUM_SLASH = 39,
    NUM_ASTERISK = 40,
    NUM_MINUS = 41,

    TAB = 42,
    Q = 43,
    W = 44,
    E = 45,
    R = 46,
    T = 47,
    Y = 48,
    U = 49,
    I = 50,
    O = 51,
    P = 52,
    OPEN_BRACKET = 53,
    CLOSE_BRACKET = 54,
    //BACKSLASH          = 55,
    KEYBOARD_DELETE = 56,
    END = 57,
    PAGE_DOWN = 58,
    NUM_SEVEN = 59,
    NUM_EIGHT = 60,
    NUM_NINE = 61,
    NUM_PLUS = 62,

    CAPS_LOCK = 63,
    A = 64,
    S = 65,
    D = 66,
    F = 67,
    G = 68,
    H = 69,
    J = 70,
    K = 71,
    L = 72,
    SEMICOLON = 73,
    APOSTROPHE = 74,
    NON_US_ASTERISK = 75,
    ENTER = 76,
    NUM_FOUR = 80,
    NUM_FIVE = 81,
    NUM_SIX = 82,

    LEFT_SHIFT = 84,
    NON_US_SLASH = 85,
    Z = 86,
    X = 87,
    C = 88,
    V = 89,
    B = 90,
    N = 91,
    M = 92,
    COMMA = 93,
    PERIOD = 94,
    FORWARD_SLASH = 95,
    RIGHT_SHIFT = 97,
    ARROW_UP = 99,
    NUM_ONE = 101,
    NUM_TWO = 102,
    NUM_THREE = 103,
    NUM_ENTER = 104,

    LEFT_CONTROL = 105,
    LEFT_WINDOWS = 106,
    LEFT_ALT = 107,
    SPACE = 110,
    RIGHT_ALT = 116,
    RIGHT_WINDOWS = 117,
    APPLICATION_SELECT = 118,
    RIGHT_CONTROL = 119,
    ARROW_LEFT = 120,
    ARROW_DOWN = 121,
    ARROW_RIGHT = 122,
    NUM_ZERO = 123,
    NUM_PERIOD = 124,

    G_1,
    G_2,
    G_3,
    G_4,
    G_5,
    G_6,
    G_7,
    G_8,
    G_9,
    G_LOGO,
    G_BADGE,
}

impl Key {
    fn from_str(string: &str) -> Option<Key> {
        let res = match string {
            "ESC" => Key::ESC,
            "F1" => Key::F1,
            "F2" => Key::F2,
            "F3" => Key::F3,
            "F4" => Key::F4,
            "F5" => Key::F5,
            "F6" => Key::F6,
            "F7" => Key::F7,
            "F8" => Key::F8,
            "F9" => Key::F9,
            "F10" => Key::F10,
            "F11" => Key::F11,
            "F12" => Key::F12,
            "PRINT_SCREEN" => Key::PRINT_SCREEN,
            "SCROLL_LOCK" => Key::SCROLL_LOCK,
            "PAUSE_BREAK" => Key::PAUSE_BREAK,

            "TILDE" => Key::TILDE,
            "ONE" => Key::ONE,
            "TWO" => Key::TWO,
            "THREE" => Key::THREE,
            "FOUR" => Key::FOUR,
            "FIVE" => Key::FIVE,
            "SIX" => Key::SIX,
            "SEVEN" => Key::SEVEN,
            "EIGHT" => Key::EIGHT,
            "NINE" => Key::NINE,
            "ZERO" => Key::ZERO,
            "MINUS" => Key::MINUS,
            "EQUALS" => Key::EQUALS,
            "BACKSPACE" => Key::BACKSPACE,
            "INSERT" => Key::INSERT,
            "HOME" => Key::HOME,
            "PAGE_UP" => Key::PAGE_UP,
            "NUM_LOCK" => Key::NUM_LOCK,
            "NUM_SLASH" => Key::NUM_SLASH,
            "NUM_ASTERISK" => Key::NUM_ASTERISK,
            "NUM_MINUS" => Key::NUM_MINUS,

            "TAB" => Key::TAB,
            "Q" => Key::Q,
            "W" => Key::W,
            "E" => Key::E,
            "R" => Key::R,
            "T" => Key::T,
            "Y" => Key::Y,
            "U" => Key::U,
            "I" => Key::I,
            "O" => Key::O,
            "P" => Key::P,
            "OPEN_BRACKET" => Key::OPEN_BRACKET,
            "CLOSE_BRACKET" => Key::CLOSE_BRACKET,
            //"BACKSLASH"           => Key::BACKSLASH,
            "KEYBOARD_DELETE" => Key::KEYBOARD_DELETE,
            "END" => Key::END,
            "PAGE_DOWN" => Key::PAGE_DOWN,
            "NUM_SEVEN" => Key::NUM_SEVEN,
            "NUM_EIGHT" => Key::NUM_EIGHT,
            "NUM_NINE" => Key::NUM_NINE,
            "NUM_PLUS" => Key::NUM_PLUS,

            "CAPS_LOCK" => Key::CAPS_LOCK,
            "A" => Key::A,
            "S" => Key::S,
            "D" => Key::D,
            "F" => Key::F,
            "G" => Key::G,
            "H" => Key::H,
            "J" => Key::J,
            "K" => Key::K,
            "L" => Key::L,
            "SEMICOLON" => Key::SEMICOLON,
            "APOSTROPHE" => Key::APOSTROPHE,
            "NON_US_ASTERISK" => Key::NON_US_ASTERISK,
            "ENTER" => Key::ENTER,
            "NUM_FOUR" => Key::NUM_FOUR,
            "NUM_FIVE" => Key::NUM_FIVE,
            "NUM_SIX" => Key::NUM_SIX,

            "LEFT_SHIFT" => Key::LEFT_SHIFT,
            "NON_US_SLASH" => Key::NON_US_SLASH,
            "Z" => Key::Z,
            "X" => Key::X,
            "C" => Key::C,
            "V" => Key::V,
            "B" => Key::B,
            "N" => Key::N,
            "M" => Key::M,
            "COMMA" => Key::COMMA,
            "PERIOD" => Key::PERIOD,
            "FORWARD_SLASH" => Key::FORWARD_SLASH,
            "RIGHT_SHIFT" => Key::RIGHT_SHIFT,
            "ARROW_UP" => Key::ARROW_UP,
            "NUM_ONE" => Key::NUM_ONE,
            "NUM_TWO" => Key::NUM_TWO,
            "NUM_THREE" => Key::NUM_THREE,
            "NUM_ENTER" => Key::NUM_ENTER,

            "LEFT_CONTROL" => Key::LEFT_CONTROL,
            "LEFT_WINDOWS" => Key::LEFT_WINDOWS,
            "LEFT_ALT" => Key::LEFT_ALT,
            "SPACE" => Key::SPACE,
            "RIGHT_ALT" => Key::RIGHT_ALT,
            "RIGHT_WINDOWS" => Key::RIGHT_WINDOWS,
            "APPLICATION_SELECT" => Key::APPLICATION_SELECT,
            "RIGHT_CONTROL" => Key::RIGHT_CONTROL,
            "ARROW_LEFT" => Key::ARROW_LEFT,
            "ARROW_DOWN" => Key::ARROW_DOWN,
            "ARROW_RIGHT" => Key::ARROW_RIGHT,
            "NUM_ZERO" => Key::NUM_ZERO,
            "NUM_PERIOD" => Key::NUM_PERIOD,

            "G1" => Key::G_1,
            "G2" => Key::G_2,
            "G3" => Key::G_3,
            "G4" => Key::G_4,
            "G5" => Key::G_5,
            "G6" => Key::G_6,
            "G7" => Key::G_7,
            "G8" => Key::G_8,
            "G9" => Key::G_9,
            "Logo" => Key::G_LOGO,
            "Badge" => Key::G_BADGE,
            _ => return None,
        };
        Some(res)
    }
    pub fn allkeys() -> Vec<Key> {
        vec![
            Key::ESC,
            Key::F1,
            Key::F2,
            Key::F3,
            Key::F4,
            Key::F5,
            Key::F6,
            Key::F7,
            Key::F8,
            Key::F9,
            Key::F10,
            Key::F11,
            Key::F12,
            Key::PRINT_SCREEN,
            Key::SCROLL_LOCK,
            Key::PAUSE_BREAK,
            Key::TILDE,
            Key::ONE,
            Key::TWO,
            Key::THREE,
            Key::FOUR,
            Key::FIVE,
            Key::SIX,
            Key::SEVEN,
            Key::EIGHT,
            Key::NINE,
            Key::ZERO,
            Key::MINUS,
            Key::EQUALS,
            Key::BACKSPACE,
            Key::INSERT,
            Key::HOME,
            Key::PAGE_UP,
            Key::NUM_LOCK,
            Key::NUM_SLASH,
            Key::NUM_ASTERISK,
            Key::NUM_MINUS,
            Key::TAB,
            Key::Q,
            Key::W,
            Key::E,
            Key::R,
            Key::T,
            Key::Y,
            Key::U,
            Key::I,
            Key::O,
            Key::P,
            Key::OPEN_BRACKET,
            Key::CLOSE_BRACKET,
            //Key::BACKSLASH,
            Key::KEYBOARD_DELETE,
            Key::END,
            Key::PAGE_DOWN,
            Key::NUM_SEVEN,
            Key::NUM_EIGHT,
            Key::NUM_NINE,
            Key::NUM_PLUS,
            Key::CAPS_LOCK,
            Key::A,
            Key::S,
            Key::D,
            Key::F,
            Key::G,
            Key::H,
            Key::J,
            Key::K,
            Key::L,
            Key::SEMICOLON,
            Key::APOSTROPHE,
            Key::NON_US_ASTERISK,
            Key::ENTER,
            Key::NUM_FOUR,
            Key::NUM_FIVE,
            Key::NUM_SIX,
            Key::LEFT_SHIFT,
            Key::NON_US_SLASH,
            Key::Z,
            Key::X,
            Key::C,
            Key::V,
            Key::B,
            Key::N,
            Key::M,
            Key::COMMA,
            Key::PERIOD,
            Key::FORWARD_SLASH,
            Key::RIGHT_SHIFT,
            Key::ARROW_UP,
            Key::NUM_ONE,
            Key::NUM_TWO,
            Key::NUM_THREE,
            Key::NUM_ENTER,
            Key::LEFT_CONTROL,
            Key::LEFT_WINDOWS,
            Key::LEFT_ALT,
            Key::SPACE,
            Key::RIGHT_ALT,
            Key::RIGHT_WINDOWS,
            Key::APPLICATION_SELECT,
            Key::RIGHT_CONTROL,
            Key::ARROW_LEFT,
            Key::ARROW_DOWN,
            Key::ARROW_RIGHT,
            Key::NUM_ZERO,
            Key::NUM_PERIOD,
            Key::G_1,
            Key::G_2,
            Key::G_3,
            Key::G_4,
            Key::G_5,
            Key::G_6,
            Key::G_7,
            Key::G_8,
            Key::G_9,
            Key::G_LOGO,
            Key::G_BADGE,
        ]
    }


    pub fn firstline() -> Vec<Key> {vec![
        Key::G_LOGO,
        Key::ESC,
        Key::F1,
        Key::F2,
        Key::F3,
        Key::F4,
        Key::F5,
        Key::F6,
        Key::F7,
        Key::F8,
        Key::F9,
        Key::F10,
        Key::F11,
        Key::F12,
        Key::PRINT_SCREEN,
        Key::SCROLL_LOCK,
        Key::PAUSE_BREAK,
    ]}

    pub fn secondline() -> Vec<Key> {vec![
        Key::G_1,
        Key::TILDE,
        Key::ONE,
        Key::TWO,
        Key::THREE,
        Key::FOUR,
        Key::FIVE,
        Key::SIX,
        Key::SEVEN,
        Key::EIGHT,
        Key::NINE,
        Key::ZERO,
        Key::MINUS,
        Key::EQUALS,
        Key::BACKSPACE,
        Key::INSERT,
        Key::HOME,
        Key::PAGE_UP,
        Key::NUM_LOCK,
        Key::NUM_SLASH,
        Key::NUM_ASTERISK,
        Key::NUM_MINUS,
    ]}

    pub fn thirdline() -> Vec<Key> {vec![
        Key::G_2,
        Key::TAB,
        Key::Q,
        Key::W,
        Key::E,
        Key::R,
        Key::T,
        Key::Y,
        Key::U,
        Key::I,
        Key::O,
        Key::P,
        Key::OPEN_BRACKET,
        Key::CLOSE_BRACKET,
        //Key::BACKSLASH,
        Key::ENTER,
        Key::KEYBOARD_DELETE,
        Key::END,
        Key::PAGE_DOWN,
        Key::NUM_SEVEN,
        Key::NUM_EIGHT,
        Key::NUM_NINE,
        Key::NUM_PLUS,
    ]}

    pub fn fourthline() -> Vec<Key> {vec![
        Key::G_3,
        Key::CAPS_LOCK,
        Key::A,
        Key::S,
        Key::D,
        Key::F,
        Key::G,
        Key::H,
        Key::J,
        Key::K,
        Key::L,
        Key::SEMICOLON,
        Key::APOSTROPHE,
        Key::NON_US_ASTERISK,
        Key::ENTER,
        Key::NUM_FOUR,
        Key::NUM_FIVE,
        Key::NUM_SIX,
        Key::NUM_PLUS,
    ]}

    pub fn fifthline() -> Vec<Key> {vec![
        Key::G_4,
        Key::LEFT_SHIFT,
        Key::NON_US_SLASH,
        Key::Z,
        Key::X,
        Key::C,
        Key::V,
        Key::B,
        Key::N,
        Key::M,
        Key::COMMA,
        Key::PERIOD,
        Key::FORWARD_SLASH,
        Key::RIGHT_SHIFT,
        Key::ARROW_UP,
        Key::NUM_ONE,
        Key::NUM_TWO,
        Key::NUM_THREE,
        Key::NUM_ENTER,
    ]}

    pub fn sixthline() -> Vec<Key> {vec![
        Key::G_5,
        Key::LEFT_CONTROL,
        Key::LEFT_WINDOWS,
        Key::LEFT_ALT,
        Key::SPACE,
        Key::RIGHT_ALT,
        Key::RIGHT_WINDOWS,
        Key::APPLICATION_SELECT,
        Key::RIGHT_CONTROL,
        Key::ARROW_LEFT,
        Key::ARROW_DOWN,
        Key::ARROW_RIGHT,
        Key::NUM_ZERO,
        Key::NUM_PERIOD,
        Key::NUM_ENTER,
    ]}

    pub fn gline() -> Vec<Key> {vec![
        Key::G_6,
        Key::G_7,
        Key::G_8,
        Key::G_9,
    ]}

    pub fn gbadge() -> Vec<Key> {vec![
        Key::G_BADGE
    ]}

}

pub fn to_logitechkey(k: &Key) -> led::Key {
    let res = match k {
        Key::ESC => led::Key::ESC,
        Key::F1 => led::Key::F1,
        Key::F2 => led::Key::F2,
        Key::F3 => led::Key::F3,
        Key::F4 => led::Key::F4,
        Key::F5 => led::Key::F5,
        Key::F6 => led::Key::F6,
        Key::F7 => led::Key::F7,
        Key::F8 => led::Key::F8,
        Key::F9 => led::Key::F9,
        Key::F10 => led::Key::F10,
        Key::F11 => led::Key::F11,
        Key::F12 => led::Key::F12,
        Key::PRINT_SCREEN => led::Key::PRINT_SCREEN,
        Key::SCROLL_LOCK => led::Key::SCROLL_LOCK,
        Key::PAUSE_BREAK => led::Key::PAUSE_BREAK,

        Key::TILDE => led::Key::TILDE,
        Key::ONE => led::Key::ONE,
        Key::TWO => led::Key::TWO,
        Key::THREE => led::Key::THREE,
        Key::FOUR => led::Key::FOUR,
        Key::FIVE => led::Key::FIVE,
        Key::SIX => led::Key::SIX,
        Key::SEVEN => led::Key::SEVEN,
        Key::EIGHT => led::Key::EIGHT,
        Key::NINE => led::Key::NINE,
        Key::ZERO => led::Key::ZERO,
        Key::MINUS => led::Key::MINUS,
        Key::EQUALS => led::Key::EQUALS,
        Key::BACKSPACE => led::Key::BACKSPACE,
        Key::INSERT => led::Key::INSERT,
        Key::HOME => led::Key::HOME,
        Key::PAGE_UP => led::Key::PAGE_UP,
        Key::NUM_LOCK => led::Key::NUM_LOCK,
        Key::NUM_SLASH => led::Key::NUM_SLASH,
        Key::NUM_ASTERISK => led::Key::NUM_ASTERISK,
        Key::NUM_MINUS => led::Key::NUM_MINUS,

        Key::TAB => led::Key::TAB,
        Key::Q => led::Key::Q,
        Key::W => led::Key::W,
        Key::E => led::Key::E,
        Key::R => led::Key::R,
        Key::T => led::Key::T,
        Key::Y => led::Key::Y,
        Key::U => led::Key::U,
        Key::I => led::Key::I,
        Key::O => led::Key::O,
        Key::P => led::Key::P,
        Key::OPEN_BRACKET => led::Key::OPEN_BRACKET,
        Key::CLOSE_BRACKET => led::Key::CLOSE_BRACKET,
        //Key::BACKSLASH           =>led:: Key::BACKSLASH,
        Key::KEYBOARD_DELETE => led::Key::KEYBOARD_DELETE,
        Key::END => led::Key::END,
        Key::PAGE_DOWN => led::Key::PAGE_DOWN,
        Key::NUM_SEVEN => led::Key::NUM_SEVEN,
        Key::NUM_EIGHT => led::Key::NUM_EIGHT,
        Key::NUM_NINE => led::Key::NUM_NINE,
        Key::NUM_PLUS => led::Key::NUM_PLUS,

        Key::CAPS_LOCK => led::Key::CAPS_LOCK,
        Key::A => led::Key::A,
        Key::S => led::Key::S,
        Key::D => led::Key::D,
        Key::F => led::Key::F,
        Key::G => led::Key::G,
        Key::H => led::Key::H,
        Key::J => led::Key::J,
        Key::K => led::Key::K,
        Key::L => led::Key::L,
        Key::SEMICOLON => led::Key::SEMICOLON,
        Key::APOSTROPHE => led::Key::APOSTROPHE,
        Key::NON_US_ASTERISK => led::Key::NON_US_SLASH, //<<<< no code in logitech lib do not use
        Key::ENTER => led::Key::ENTER,
        Key::NUM_FOUR => led::Key::NUM_FOUR,
        Key::NUM_FIVE => led::Key::NUM_FIVE,
        Key::NUM_SIX => led::Key::NUM_SIX,

        Key::LEFT_SHIFT => led::Key::LEFT_SHIFT,
        Key::NON_US_SLASH => led::Key::NON_US_SLASH,
        Key::Z => led::Key::Z,
        Key::X => led::Key::X,
        Key::C => led::Key::C,
        Key::V => led::Key::V,
        Key::B => led::Key::B,
        Key::N => led::Key::N,
        Key::M => led::Key::M,
        Key::COMMA => led::Key::COMMA,
        Key::PERIOD => led::Key::PERIOD,
        Key::FORWARD_SLASH => led::Key::FORWARD_SLASH,
        Key::RIGHT_SHIFT => led::Key::RIGHT_SHIFT,
        Key::ARROW_UP => led::Key::ARROW_UP,
        Key::NUM_ONE => led::Key::NUM_ONE,
        Key::NUM_TWO => led::Key::NUM_TWO,
        Key::NUM_THREE => led::Key::NUM_THREE,
        Key::NUM_ENTER => led::Key::NUM_ENTER,

        Key::LEFT_CONTROL => led::Key::LEFT_CONTROL,
        Key::LEFT_WINDOWS => led::Key::LEFT_WINDOWS,
        Key::LEFT_ALT => led::Key::LEFT_ALT,
        Key::SPACE => led::Key::SPACE,
        Key::RIGHT_ALT => led::Key::RIGHT_ALT,
        Key::RIGHT_WINDOWS => led::Key::RIGHT_WINDOWS,
        Key::APPLICATION_SELECT => led::Key::APPLICATION_SELECT,
        Key::RIGHT_CONTROL => led::Key::RIGHT_CONTROL,
        Key::ARROW_LEFT => led::Key::ARROW_LEFT,
        Key::ARROW_DOWN => led::Key::ARROW_DOWN,
        Key::ARROW_RIGHT => led::Key::ARROW_RIGHT,
        Key::NUM_ZERO => led::Key::NUM_ZERO,
        Key::NUM_PERIOD => led::Key::NUM_PERIOD,

        Key::G_1 => led::Key::G_1,
        Key::G_2 => led::Key::G_2,
        Key::G_3 => led::Key::G_3,
        Key::G_4 => led::Key::G_4,
        Key::G_5 => led::Key::G_5,
        Key::G_6 => led::Key::G_6,
        Key::G_7 => led::Key::G_7,
        Key::G_8 => led::Key::G_8,
        Key::G_9 => led::Key::G_9,
        Key::G_LOGO => led::Key::G_LOGO,
        Key::G_BADGE => led::Key::G_BADGE,
    };
    res
}

pub fn scancode_to_key(sc: u8) -> Option<Key> {
    let sc = klogi::keys::scancode_to_keycode(sc)?;
    let res = match sc {
        1   /*KEY_ESC */        => Key::ESC,
        2   /*KEY_1 */          => Key::ONE,
        3   /*KEY_2 */          => Key::TWO,
        4   /*KEY_3 */          => Key::THREE,
        5   /*KEY_4 */          => Key::FOUR,
        6   /*KEY_5 */          => Key::FIVE,
        7   /*KEY_6 */          => Key::SIX,
        8   /*KEY_7 */          => Key::SEVEN,
        9   /*KEY_8 */          => Key::EIGHT,
        10  /*KEY_9 */          => Key::NINE,
        11  /*KEY_0 */          => Key::ZERO,
        12  /*KEY_MINUS */      => Key::MINUS,
        13  /*KEY_EQUAL */      => Key::EQUALS,
        14  /*KEY_BACKSPACE */  => Key::BACKSPACE,
        15  /*KEY_TAB */        => Key::TAB,
        16  /*KEY_Q */          => Key::Q,
        17  /*KEY_W */          => Key::W,
        18  /*KEY_E */          => Key::E,
        19  /*KEY_R */          => Key::R,
        20  /*KEY_T */          => Key::T,
        21  /*KEY_Y */          => Key::Y,
        22  /*KEY_U */          => Key::U,
        23  /*KEY_I */          => Key::I,
        24  /*KEY_O */          => Key::O,
        25  /*KEY_P */          => Key::P,
        26  /*KEY_LEFTBRACE */  => Key::OPEN_BRACKET,
        27  /*KEY_RIGHTBRACE */ => Key::CLOSE_BRACKET,
        28  /*KEY_ENTER */      => Key::ENTER,
        29  /*KEY_LEFTCTRL */   => Key::LEFT_CONTROL,
        30  /*KEY_A */          => Key::A,
        31  /*KEY_S */          => Key::S,
        32  /*KEY_D */          => Key::D,
        33  /*KEY_F */          => Key::F,
        34  /*KEY_G */          => Key::G,
        35  /*KEY_H */          => Key::H,
        36  /*KEY_J */          => Key::J,
        37  /*KEY_K */          => Key::K,
        38  /*KEY_L */          => Key::L,
        39  /*KEY_SEMICOLON */  => Key::SEMICOLON,
        40  /*KEY_APOSTROPHE */ => Key::APOSTROPHE,
        41  /*KEY_GRAVE */      => Key::TILDE,
        42  /*KEY_LEFTSHIFT */  => Key::LEFT_SHIFT,
        43  /*KEY_BACKSLASH */  => Key::NON_US_ASTERISK,
        44  /*KEY_Z */          => Key::Z,
        45  /*KEY_X */          => Key::X,
        46  /*KEY_C */          => Key::C,
        47  /*KEY_V */          => Key::V,
        48  /*KEY_B */          => Key::B,
        49  /*KEY_N */          => Key::N,
        50  /*KEY_M */          => Key::M,
        51  /*KEY_COMMA */      => Key::COMMA,
        52  /*KEY_DOT */        => Key::PERIOD,
        53  /*KEY_SLASH */      => Key::FORWARD_SLASH,
        54  /*KEY_RIGHTSHIFT */ => Key::RIGHT_SHIFT,
        55  /*KEY_KPASTERISK */ => Key::NUM_ASTERISK,
        56  /*KEY_LEFTALT */    => Key::LEFT_ALT,
        57  /*KEY_SPACE */      => Key::SPACE,
        58  /*KEY_CAPSLOCK */   => Key::CAPS_LOCK,
        59  /*KEY_F1 */         => Key::F1,
        60  /*KEY_F2 */         => Key::F2,
        61  /*KEY_F3 */         => Key::F3,
        62  /*KEY_F4 */         => Key::F4,
        63  /*KEY_F5 */         => Key::F5,
        64  /*KEY_F6 */         => Key::F6,
        65  /*KEY_F7 */         => Key::F7,
        66  /*KEY_F8 */         => Key::F8,
        67  /*KEY_F9 */         => Key::F9,
        68  /*KEY_F10 */        => Key::F10,
        69  /*KEY_NUMLOCK */    => Key::NUM_LOCK,
        70  /*KEY_SCROLLLOCK */ => Key::SCROLL_LOCK,
        71  /*KEY_KP7 */        => Key::NUM_SEVEN,
        72  /*KEY_KP8 */        => Key::NUM_EIGHT,
        73  /*KEY_KP9 */        => Key::NUM_NINE,
        74  /*KEY_KPMINUS */    => Key::NUM_MINUS,
        75  /*KEY_KP4 */        => Key::NUM_FOUR,
        76  /*KEY_KP5 */        => Key::NUM_FIVE,
        77  /*KEY_KP6 */        => Key::NUM_SIX,
        78  /*KEY_KPPLUS */     => Key::NUM_PLUS,
        79  /*KEY_KP1 */        => Key::NUM_ONE,
        80  /*KEY_KP2 */        => Key::NUM_TWO,
        81  /*KEY_KP3 */        => Key::NUM_THREE,
        82  /*KEY_KP0 */        => Key::NUM_ZERO,
        83  /*KEY_KPDOT */      => Key::NUM_PERIOD,
        110 /*KEY_INSERT */     => Key::INSERT,
        111 /*KEY_DELETE */     => Key::KEYBOARD_DELETE,
        107 /*KEY_END */        => Key::END,
        102 /*KEY_HOME */       => Key::HOME,
        100 /*KEY_RIGHTALT */   => Key::RIGHT_ALT,
        97  /*KEY_RIGHTCTRL */  => Key::RIGHT_CONTROL,
        87  /*KEY_F11 */        => Key::F11,
        88  /*KEY_F12 */        => Key::F12,
        103 /*KEY_UP */         => Key::ARROW_UP,
        104 /*KEY_PAGEUP */     => Key::PAGE_UP,
        105 /*KEY_LEFT */       => Key::ARROW_LEFT,
        106 /*KEY_RIGHT */      => Key::ARROW_RIGHT,
        108 /*KEY_DOWN */       => Key::ARROW_DOWN,
        109 /*KEY_PAGEDOWN */   => Key::PAGE_DOWN,
        119 /*KEY_PAUSE */      => Key::PAUSE_BREAK,
        125 /*KEY_LEFTMETA */   => Key::LEFT_WINDOWS,
        126 /*KEY_RIGHTMETA */  => Key::RIGHT_WINDOWS,
        99  /*KEY_SYSRQ */      => Key::PRINT_SCREEN,
        127 /*KEY_COMPOSE */    => Key::APPLICATION_SELECT,
        98  /*KEY_KPSLASH */    => Key::NUM_SLASH,
        96  /*KEY_KPENTER */    => Key::NUM_ENTER,
        86  /*KEY_102ND */      => Key::NON_US_SLASH,


        //85  /*KEY_ZENKAKUHANKAKU */ => Key::ZENKAKUHANKAKU,
        //86  /*KEY_102ND */ => Key::102ND,
        //89  /*KEY_RO */ => Key::RO,
        //90  /*KEY_KATAKANA */ => Key::KATAKANA,
        //91  /*KEY_HIRAGANA */ => Key::HIRAGANA,
        //92  /*KEY_HENKAN */ => Key::HENKAN,
        //93  /*KEY_KATAKANAHIRAGANA */ => Key::KATAKANAHIRAGANA,
        //94  /*KEY_MUHENKAN */ => Key::MUHENKAN,
        //95  /*KEY_KPJPCOMMA */ => Key::KPJPCOMMA,
        //101 /*KEY_LINEFEED */ => Key::LINEFEED,
        //112 /*KEY_MACRO */ => Key::MACRO,
        //113 /*KEY_MUTE */ => Key::MUTE,
        //114 /*KEY_VOLUMEDOWN */ => Key::VOLUMEDOWN,
        //115 /*KEY_VOLUMEUP */ => Key::VOLUMEUP,
        //116 /*KEY_POWER  SC System Power Down */ => Key::POWER,
        //117 /*KEY_KPEQUAL */ => Key::KPEQUAL,
        //118 /*KEY_KPPLUSMINUS */ => Key::KPPLUSMINUS,
        //120 /*KEY_SCALE  AL Compiz Scale (Expose) */ => Key::SCALE,
        //121 /*KEY_KPCOMMA */ => Key::KPCOMMA,
        //122 /*KEY_HANGEUL */ => Key::HANGEUL,
        //KEY_HANGEUL /*KEY_HANGUEL */ => Key::HANGUEL,
        //123 /*KEY_HANJA */ => Key::HANJA,
        //124 /*KEY_YEN */ => Key::YEN,
        //128 /*KEY_STOP  AC Stop */ => Key::STOP,
        //129 /*KEY_AGAIN */ => Key::AGAIN,
        //130 /*KEY_PROPS  AC Properties */ => Key::PROPS,
        //131 /*KEY_UNDO  AC Undo */ => Key::UNDO,
        //132 /*KEY_FRONT */ => Key::FRONT,
        //133 /*KEY_COPY  AC Copy */ => Key::COPY,
        //134 /*KEY_OPEN  AC Open */ => Key::OPEN,
        //135 /*KEY_PASTE  AC Paste */ => Key::PASTE,
        //136 /*KEY_FIND  AC Search */ => Key::FIND,
        //137 /*KEY_CUT  AC Cut */ => Key::CUT,
        //138 /*KEY_HELP  AL Integrated Help Center */ => Key::HELP,
        //139 /*KEY_MENU  Menu (show menu) */ => Key::MENU,
        //140 /*KEY_CALC  AL Calculator */ => Key::CALC,
        //141 /*KEY_SETUP */ => Key::SETUP,
        //142 /*KEY_SLEEP  SC System Sleep */ => Key::SLEEP,
        //143 /*KEY_WAKEUP  System Wake Up */ => Key::WAKEUP,
        //144 /*KEY_FILE  AL Local Machine Browser */ => Key::FILE,
        //145 /*KEY_SENDFILE */ => Key::SENDFILE,
        //146 /*KEY_DELETEFILE */ => Key::DELETEFILE,
        //147 /*KEY_XFER */ => Key::XFER,
        //148 /*KEY_PROG1 */ => Key::PROG1,
        //149 /*KEY_PROG2 */ => Key::PROG2,
        //150 /*KEY_WWW  AL Internet Browser */ => Key::WWW,
        //151 /*KEY_MSDOS */ => Key::MSDOS,
        //152 /*KEY_COFFEE  AL Terminal Lock/Screensaver */ => Key::COFFEE,
        //KEY_COFFEE /*KEY_SCREENLOCK */ => Key::SCREENLOCK,
        //153 /*KEY_ROTATE_DISPLAY  Display orientation for e.g. tablets */ => Key::ROTATE_DISPLAY,
        //KEY_ROTATE_DISPLAY /*KEY_DIRECTION */ => Key::DIRECTION,
        //154 /*KEY_CYCLEWINDOWS */ => Key::CYCLEWINDOWS,
        //155 /*KEY_MAIL */ => Key::MAIL,
        //156 /*KEY_BOOKMARKS  AC Bookmarks */ => Key::BOOKMARKS,
        //157 /*KEY_COMPUTER */ => Key::COMPUTER,
        //158 /*KEY_BACK  AC Back */ => Key::BACK,
        //159 /*KEY_FORWARD  AC Forward */ => Key::FORWARD,
        //160 /*KEY_CLOSECD */ => Key::CLOSECD,
        //161 /*KEY_EJECTCD */ => Key::EJECTCD,
        //162 /*KEY_EJECTCLOSECD */ => Key::EJECTCLOSECD,
        //163 /*KEY_NEXTSONG */ => Key::NEXTSONG,
        //164 /*KEY_PLAYPAUSE */ => Key::PLAYPAUSE,
        //165 /*KEY_PREVIOUSSONG */ => Key::PREVIOUSSONG,
        //166 /*KEY_STOPCD */ => Key::STOPCD,
        //167 /*KEY_RECORD */ => Key::RECORD,
        //168 /*KEY_REWIND */ => Key::REWIND,
        //169 /*KEY_PHONE  Media Select Telephone */ => Key::PHONE,
        //170 /*KEY_ISO */ => Key::ISO,
        //171 /*KEY_CONFIG  AL Consumer Control Configuration */ => Key::CONFIG,
        //172 /*KEY_HOMEPAGE  AC Home */ => Key::HOMEPAGE,
        //173 /*KEY_REFRESH  AC Refresh */ => Key::REFRESH,
        //174 /*KEY_EXIT  AC Exit */ => Key::EXIT,
        //175 /*KEY_MOVE */ => Key::MOVE,
        //176 /*KEY_EDIT */ => Key::EDIT,
        //177 /*KEY_SCROLLUP */ => Key::SCROLLUP,
        //178 /*KEY_SCROLLDOWN */ => Key::SCROLLDOWN,
        //179 /*KEY_KPLEFTPAREN */ => Key::KPLEFTPAREN,
        //180 /*KEY_KPRIGHTPAREN */ => Key::KPRIGHTPAREN,
        //181 /*KEY_NEW  AC New */ => Key::NEW,

        _ => return None
    };
    Some(res)
}

pub struct KeyboardLayout {
    keys_db: KeysDB,
}
impl KeyboardLayout {
    pub fn new() -> Self {
        let kl = klogi::parse_layout();
        let mut keylayout = kl
            .zones
            .clone()
            .into_iter()
            .map(|z| {
                z.keys.into_iter().filter_map(move |z1| {
                    if let Some(code) = scancode_to_key(z1.code as u8) {
                        Some((code, z1.clone()))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect::<HashMap<_, _>>();

        keylayout.extend(
            kl.zones
                .into_iter()
                .map(|z| {
                    z.keys.into_iter().filter_map(move |z1| {
                        if let Some(glyph) = &z1.glyph {
                            if let Some(gkey) = Key::from_str(glyph) {
                                Some((gkey, z1.clone()))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                })
                .flatten()
                .collect::<HashMap<_, _>>(),
        );
        Self {
            keys_db: keylayout,
        }
    }

    pub fn get_keydata(&self, key: Key) -> KeydataType {
        (key, self.keys_db[&key].clone())
    }
    pub fn get_keysdata(&self, keys: Vec<Key>) -> Vec<KeydataType> {
        let mut v = Vec::with_capacity(keys.len());
        for k in keys {
            v.push((k, self.keys_db[&k].clone()));
        }
        return v;
    }

}

/*
#[repr(C)]
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types, non_snake_case)]
pub enum Key_fr {
    ESC,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PRINT_SCREEN,
    SCROLL_LOCK,
    PAUSE_BREAK,

    TILDE,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    ZERO,
    MINUS,
    EQUALS,
    BACKSPACE,
    INSERT,
    HOME,
    PAGE_UP,
    NUM_LOCK,
    NUM_SLASH,
    NUM_ASTERISK,
    NUM_MINUS,

    TAB,
    A,
    Z,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    OPEN_BRACKET,
    CLOSE_BRACKET,
    KEYBOARD_DELETE,
    END,
    PAGE_DOWN,
    NUM_SEVEN,
    NUM_EIGHT,
    NUM_NINE,
    NUM_PLUS,

    CAPS_LOCK,
    Q,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    M,
    PERIOD,
    NON_US_ASTERISK,
    ENTER,
    NUM_FOUR,
    NUM_FIVE,
    NUM_SIX,

    LEFT_SHIFT,
    NON_US_SLASH,
    W,
    X,
    C,
    V,
    B,
    N,
    COMMA,
    SEMICOLON,
    FORWARD_SLASH,
    EXCLAMATION,
    RIGHT_SHIFT,
    ARROW_UP,
    NUM_ONE,
    NUM_TWO,
    NUM_THREE,
    NUM_ENTER,

    LEFT_CONTROL,
    LEFT_WINDOWS,
    LEFT_ALT,
    SPACE,
    RIGHT_ALT,
    RIGHT_WINDOWS,
    APPLICATION_SELECT,
    RIGHT_CONTROL,
    ARROW_LEFT,
    ARROW_DOWN,
    ARROW_RIGHT,
    NUM_ZERO,
    NUM_PERIOD,
}

impl From<Key_fr> for Key {
    fn from(k: Key_fr) -> Self {
        match k {
            Key_fr::ESC                => Key::ESC               ,
            Key_fr::F1                 => Key::F1                ,
            Key_fr::F2                 => Key::F2                ,
            Key_fr::F3                 => Key::F3                ,
            Key_fr::F4                 => Key::F4                ,
            Key_fr::F5                 => Key::F5                ,
            Key_fr::F6                 => Key::F6                ,
            Key_fr::F7                 => Key::F7                ,
            Key_fr::F8                 => Key::F8                ,
            Key_fr::F9                 => Key::F9                ,
            Key_fr::F10                => Key::F10               ,
            Key_fr::F11                => Key::F11               ,
            Key_fr::F12                => Key::F12               ,
            Key_fr::PRINT_SCREEN       => Key::PRINT_SCREEN      ,
            Key_fr::SCROLL_LOCK        => Key::SCROLL_LOCK       ,
            Key_fr::PAUSE_BREAK        => Key::PAUSE_BREAK       ,

            Key_fr::TILDE              => Key::TILDE             ,
            Key_fr::ONE                => Key::ONE               ,
            Key_fr::TWO                => Key::TWO               ,
            Key_fr::THREE              => Key::THREE             ,
            Key_fr::FOUR               => Key::FOUR              ,
            Key_fr::FIVE               => Key::FIVE              ,
            Key_fr::SIX                => Key::SIX               ,
            Key_fr::SEVEN              => Key::SEVEN             ,
            Key_fr::EIGHT              => Key::EIGHT             ,
            Key_fr::NINE               => Key::NINE              ,
            Key_fr::ZERO               => Key::ZERO              ,
            Key_fr::MINUS              => Key::MINUS             ,
            Key_fr::EQUALS             => Key::EQUALS            ,
            Key_fr::BACKSPACE          => Key::BACKSPACE         ,
            Key_fr::INSERT             => Key::INSERT            ,
            Key_fr::HOME               => Key::HOME              ,
            Key_fr::PAGE_UP            => Key::PAGE_UP           ,
            Key_fr::NUM_LOCK           => Key::NUM_LOCK          ,
            Key_fr::NUM_SLASH          => Key::NUM_SLASH         ,
            Key_fr::NUM_ASTERISK       => Key::NUM_ASTERISK      ,
            Key_fr::NUM_MINUS          => Key::NUM_MINUS         ,

            Key_fr::TAB                => Key::TAB               ,
            Key_fr::A                  => Key::Q                 ,
            Key_fr::Z                  => Key::W                 ,
            Key_fr::E                  => Key::E                 ,
            Key_fr::R                  => Key::R                 ,
            Key_fr::T                  => Key::T                 ,
            Key_fr::Y                  => Key::Y                 ,
            Key_fr::U                  => Key::U                 ,
            Key_fr::I                  => Key::I                 ,
            Key_fr::O                  => Key::O                 ,
            Key_fr::P                  => Key::P                 ,
            Key_fr::OPEN_BRACKET       => Key::OPEN_BRACKET      ,
            Key_fr::CLOSE_BRACKET      => Key::CLOSE_BRACKET     ,
            Key_fr::KEYBOARD_DELETE    => Key::KEYBOARD_DELETE   ,
            Key_fr::END                => Key::END               ,
            Key_fr::PAGE_DOWN          => Key::PAGE_DOWN         ,
            Key_fr::NUM_SEVEN          => Key::NUM_SEVEN         ,
            Key_fr::NUM_EIGHT          => Key::NUM_EIGHT         ,
            Key_fr::NUM_NINE           => Key::NUM_NINE          ,
            Key_fr::NUM_PLUS           => Key::NUM_PLUS          ,

            Key_fr::CAPS_LOCK          => Key::CAPS_LOCK         ,
            Key_fr::Q                  => Key::A                 ,
            Key_fr::S                  => Key::S                 ,
            Key_fr::D                  => Key::D                 ,
            Key_fr::F                  => Key::F                 ,
            Key_fr::G                  => Key::G                 ,
            Key_fr::H                  => Key::H                 ,
            Key_fr::J                  => Key::J                 ,
            Key_fr::K                  => Key::K                 ,
            Key_fr::L                  => Key::L                 ,
            Key_fr::M                  => Key::SEMICOLON         ,
            Key_fr::PERIOD             => Key::APOSTROPHE        ,
            Key_fr::NON_US_ASTERISK    => Key::NON_US_ASTERISK   ,
            Key_fr::ENTER              => Key::ENTER             ,
            Key_fr::NUM_FOUR           => Key::NUM_FOUR          ,
            Key_fr::NUM_FIVE           => Key::NUM_FIVE          ,
            Key_fr::NUM_SIX            => Key::NUM_SIX           ,

            Key_fr::LEFT_SHIFT         => Key::LEFT_SHIFT        ,
            Key_fr::NON_US_SLASH       => Key::NON_US_SLASH      ,
            Key_fr::W                  => Key::Z                 ,
            Key_fr::X                  => Key::X                 ,
            Key_fr::C                  => Key::C                 ,
            Key_fr::V                  => Key::V                 ,
            Key_fr::B                  => Key::B                 ,
            Key_fr::N                  => Key::N                 ,
            Key_fr::COMMA              => Key::M                 ,
            Key_fr::SEMICOLON          => Key::COMMA             ,
            Key_fr::FORWARD_SLASH      => Key::PERIOD            ,
            Key_fr::EXCLAMATION        => Key::FORWARD_SLASH     ,
            Key_fr::RIGHT_SHIFT        => Key::RIGHT_SHIFT       ,
            Key_fr::ARROW_UP           => Key::ARROW_UP          ,
            Key_fr::NUM_ONE            => Key::NUM_ONE           ,
            Key_fr::NUM_TWO            => Key::NUM_TWO           ,
            Key_fr::NUM_THREE          => Key::NUM_THREE         ,
            Key_fr::NUM_ENTER          => Key::NUM_ENTER         ,

            Key_fr::LEFT_CONTROL       => Key::LEFT_CONTROL      ,
            Key_fr::LEFT_WINDOWS       => Key::LEFT_WINDOWS      ,
            Key_fr::LEFT_ALT           => Key::LEFT_ALT          ,
            Key_fr::SPACE              => Key::SPACE             ,
            Key_fr::RIGHT_ALT          => Key::RIGHT_ALT         ,
            Key_fr::RIGHT_WINDOWS      => Key::RIGHT_WINDOWS     ,
            Key_fr::APPLICATION_SELECT => Key::APPLICATION_SELECT,
            Key_fr::RIGHT_CONTROL      => Key::RIGHT_CONTROL     ,
            Key_fr::ARROW_LEFT         => Key::ARROW_LEFT        ,
            Key_fr::ARROW_DOWN         => Key::ARROW_DOWN        ,
            Key_fr::ARROW_RIGHT        => Key::ARROW_RIGHT       ,
            Key_fr::NUM_ZERO           => Key::NUM_ZERO          ,
            Key_fr::NUM_PERIOD         => Key::NUM_PERIOD        ,

        }
    }
} */