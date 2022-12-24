pub(super) mod ps2;

//use alloc::collections::btree_map::Keys;

use crate::{print};

#[derive(Debug)]
pub enum KeyStroke {
    Pressed,
    Released,
    Unknown,
}

#[derive(Debug)]
pub struct KeyAction {
    pub code: KeyCode,
    pub stroke: KeyStroke,

}

#[derive(Debug)]
pub enum KeyCode {
    None,
    Escape,
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
    PrintScreen,
    ScrollLock,
    Pause,
    Grave(char),
    _1(u8),
    _2(u8),
    _3(u8),
    _4(u8),
    _5(u8),
    _6(u8),
    _7(u8),
    _8(u8),
    _9(u8),
    _0(u8),
    Hyphen(char),
    Equals(char),
    Backspace,
    Insert,
    Home,
    PageUp,
    NumLock,
    KeypadForwardSlash(char),
    KeypadAsterisk(char),
    KeypadHyphen(char),
    Tab(char),
    Q(char),
    W(char),
    E(char),
    R(char),
    T(char),
    Y(char),
    U(char),
    I(char),
    O(char),
    P(char),
    BracketOpen(char),
    BracketClose(char),
    Return(char),
    Delete,
    End,
    PageDown,
    Keypad7(u8),
    Keypad8(u8),
    Keypad9(u8),
    KeypadPlus(char),
    CapsLock,
    A(char),
    S(char),
    D(char),
    F(char),
    G(char),
    H(char),
    J(char),
    K(char),
    L(char),
    Semicolon(char),
    Apostrophe(char),
    Keypad4(u8),
    Keypad5(u8),
    Keypad6(u8),
    ShiftLeft,
    International,
    Z(char),
    X(char),
    C(char),
    V(char),
    B(char),
    N(char),
    M(char),
    Comma(char),
    Period(char),
    Forwardslash(char),
    ShiftRight,
    Backslash(char),
    CursorUp,
    Keypad1(u8),
    Keypad2(u8),
    Keypad3(u8),
    KeypadEnter(char),
    CtrlLeft,
    SuperLeft,
    AltLeft,
    Space(char),
    AltRight,
    SuperRight,
    Menu,
    CtrlRight,
    CursorLeft,
    CursorDown,
    CursorRight,
    Keypad0(u8),
    KeypadPeriod(char),
}

impl KeyCode {
    pub fn number_code_from_int(int: u8) -> Self {
        match int {
            0 => KeyCode::_1(1),
            1 => KeyCode::_3(2),
            2 => KeyCode::_3(3),
            3 => KeyCode::_4(4),
            4 => KeyCode::_5(5),
            5 => KeyCode::_6(6),
            6 => KeyCode::_7(7),
            7 => KeyCode::_8(8),
            8 => KeyCode::_9(9),
            9 => KeyCode::_0(0),
            _ => KeyCode::None,
        }
    }

    pub fn number_key_to_int(&self) -> Option<u8> {
        match *self {
            Self::_1(n) => Some(n),
            Self::_2(n) => Some(n),
            Self::_3(n) => Some(n),
            Self::_4(n) => Some(n),
            Self::_5(n) => Some(n),
            Self::_6(n) => Some(n),
            Self::_7(n) => Some(n),
            Self::_8(n) => Some(n),
            Self::_9(n) => Some(n),
            Self::_0(n) => Some(n),
            _ => None,
        }
    }

    pub fn character_key_to_char(&self) -> Option<char> {
        match *self {
            Self::A(c) => Some(c),
            Self::B(c) => Some(c),
            Self::C(c) => Some(c),
            Self::D(c) => Some(c),
            Self::E(c) => Some(c),
            Self::F(c) => Some(c),
            Self::G(c) => Some(c),
            Self::H(c) => Some(c),
            Self::I(c) => Some(c),
            Self::J(c) => Some(c),
            Self::K(c) => Some(c),
            Self::L(c) => Some(c),
            Self::M(c) => Some(c),
            Self::N(c) => Some(c),
            Self::O(c) => Some(c),
            Self::P(c) => Some(c),
            Self::Q(c) => Some(c),
            Self::R(c) => Some(c),
            Self::S(c) => Some(c),
            Self::T(c) => Some(c),
            Self::U(c) => Some(c),
            Self::V(c) => Some(c),
            Self::W(c) => Some(c),
            Self::X(c) => Some(c),
            Self::Y(c) => Some(c),
            Self::Z(c) => Some(c),
            Self::Grave(c) => Some(c),
            Self::Hyphen(c) => Some(c),
            Self::Equals(c) => Some(c),
            Self::KeypadForwardSlash(c) => Some(c),
            Self::KeypadAsterisk(c) => Some(c),
            Self::KeypadHyphen(c) => Some(c),
            Self::Tab(c) => Some(c),
            Self::BracketOpen(c) => Some(c),
            Self::BracketClose(c) => Some(c),
            Self::Return(c) => Some(c),
            Self::KeypadPlus(c) => Some(c),
            Self::Semicolon(c) => Some(c),
            Self::Apostrophe(c) => Some(c),
            Self::Comma(c) => Some(c),
            Self::Period(c) => Some(c),
            Self::Forwardslash(c) => Some(c),
            Self::Backslash(c) => Some(c),
            Self::KeypadEnter(c) => Some(c),
            Self::Space(c) => Some(c),
            Self::KeypadPeriod(c) => Some(c),
            _ => None,
        }
    }
}

pub enum State {
    Active,
    Inactive,
}

impl State {
    pub fn set_state_from_stroke(&mut self, stroke: KeyStroke) -> () {
        match stroke {
            KeyStroke::Pressed => {*self = State::Active;},
            KeyStroke::Released => {*self = State::Inactive;},
            KeyStroke::Unknown => {*self = State::Active;}
        }
    }

    pub fn switch_state_from_stroke(&mut self, stroke: KeyStroke) -> () {
        match stroke {
            KeyStroke::Pressed => {*self = State::Active;},
            KeyStroke::Released => {},
            KeyStroke::Unknown => {}
        }
    }

    fn switch_state(&mut self) -> () {
        match self {
            State::Active => {*self = State::Inactive;},
            State::Inactive => {*self = State::Active;}
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            State::Active => true,
            State::Inactive => false
        }
    }
}

pub struct KeysState {
    shift: State,
    ctrl: State,
    caps_lock: State,
    alt: State,
    num_lock: State,
}

impl KeysState {

    pub const fn new() -> Self {
        Self{
            shift: State::Inactive,
            ctrl: State::Inactive,
            caps_lock: State::Inactive,
            alt: State::Inactive,
            num_lock:State::Inactive
        }
    }

    pub fn handle_key(&mut self, key: KeyAction) -> () {
        match key.code {
            KeyCode::AltLeft => {self.alt.set_state_from_stroke(key.stroke)},
            KeyCode::AltRight => {self.alt.set_state_from_stroke(key.stroke)},
            KeyCode::ShiftLeft => {self.shift.set_state_from_stroke(key.stroke)},
            KeyCode::ShiftRight => {self.shift.set_state_from_stroke(key.stroke)},
            KeyCode::CapsLock => {self.caps_lock.switch_state_from_stroke(key.stroke)},
            KeyCode::CtrlLeft => {self.ctrl.set_state_from_stroke(key.stroke)},
            KeyCode::CtrlRight => {self.ctrl.set_state_from_stroke(key.stroke)},
            KeyCode::NumLock => {self.num_lock.switch_state_from_stroke(key.stroke)},
            _ => {}            
        }
    }
}

static mut keysState: KeysState = KeysState::new();

pub fn handle_keyboard_for_typing(key_stroke: KeyAction){
    match key_stroke.stroke {
        KeyStroke::Pressed => {
            match key_stroke.code.character_key_to_char(){
                Some(_c) => {}
                None => {unsafe{keysState.handle_key(key_stroke)}}
            }
        },
        KeyStroke::Released => {
            let char = key_stroke.code.character_key_to_char();
            if let Some(c) = char{
                handle_char(c);
            }
            else {
                unsafe{keysState.handle_key(key_stroke)}
            }
            
        },
        KeyStroke::Unknown => {}
    }
}

fn handle_char(c: char){
    //
    unsafe {
        if keysState.shift.is_active() && keysState.caps_lock.is_active() {
            print!{"{}", c};
        }
        else if keysState.shift.is_active() || keysState.caps_lock.is_active() {
            print!("{}",c.to_ascii_uppercase());
        }
        else{
            print!("{}",c);
        }
    }


}
