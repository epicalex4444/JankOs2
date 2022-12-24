
use super::super::{in_b, out_b};
use super::{KeyStroke, KeyCode, KeyAction};

const DATA_REGISTER: u16 = 0x60;
const STATUS_REGISTER: u16 = 0x64;

// impl Index<u32> for Scancodes {
//     type Output = Scancodes;

//     fn index(&self)
// }

pub struct Ps2Controller {}

impl Ps2Controller{

    // TODO: Naive implementation, see https://wiki.osdev.org/%228042%22_PS/2_Controller for more robust intialisation
    pub const fn new() -> Ps2Controller{
        Ps2Controller{}
    }

    fn in_staus(&self) -> u8 {
        unsafe{
            in_b(STATUS_REGISTER)
        }        
    }

    fn in_data(&self) -> u8 {
        unsafe{
            in_b(DATA_REGISTER)
        }
    }

    fn out_command(&self, command: u8) -> &Self {
        unsafe{
            out_b(STATUS_REGISTER, command);
        }
        self
    }

    fn out_data(&self, data: u8) -> &Self {
        unsafe{
            out_b(DATA_REGISTER, data);
        }
        self
    }

    pub fn read_data(&self) -> u8 {
        self.in_data()
    }
    
    pub fn keystroke_from_ps2_scancode(&self, scancode:u8) -> KeyAction {
        let stroke = match scancode{
            0x1..0x58 => KeyStroke::Pressed,
            0x81..0xD8 => KeyStroke::Released,
            _ => KeyStroke::Unknown
        };

        let code: KeyCode = match stroke {
            KeyStroke::Pressed => {Ps2Controller::code_from_index(scancode, KeyStroke::Pressed)},
            KeyStroke::Released => {Ps2Controller::code_from_index(scancode, KeyStroke::Released)},
            KeyStroke::Unknown => {KeyCode::None}
        };

        return KeyAction{stroke: stroke, code: code}
    }

    fn code_from_index(i:u8, pressed: KeyStroke) -> KeyCode {

        let index_adjusted = match pressed {
            KeyStroke::Pressed => i,
            KeyStroke::Released => i - 0x80,
            KeyStroke::Unknown => 0,            
        };

        match index_adjusted {
            0x1 => KeyCode::Escape,
            0x2..0xB => KeyCode::number_code_from_int(index_adjusted - 2),
            0xC => KeyCode::Hyphen('-'),
            0xD => KeyCode::Equals('='),
            0xE => KeyCode::Backspace,
            0xF => KeyCode::Tab('\t'),
            0x10 => KeyCode::Q('q'),
            0x11 => KeyCode::W('w'),
            0x12 => KeyCode::E('e'),
            0x13 => KeyCode::R('r'),
            0x14 => KeyCode::T('t'),
            0x15 => KeyCode::Y('y'),
            0x16 => KeyCode::U('u') ,
            0x17 => KeyCode::I('i') ,
            0x18 => KeyCode::O('o') ,
            0x19 => KeyCode::P('p') ,
            0x1A => KeyCode::BracketOpen('[') ,
            0x1B => KeyCode::BracketClose(']') ,
            0x1C => KeyCode::Return('\n') ,
            0x1D => KeyCode::CtrlLeft ,
            0x1E => KeyCode::A('a') ,
            0x1F => KeyCode::S('s') ,
            0x20 => KeyCode::D('d') ,
            0x21 => KeyCode::F('f') ,
            0x22 => KeyCode::G('g') ,
            0x23 => KeyCode::H('h') ,
            0x24 => KeyCode::J('j') ,
            0x25 => KeyCode::K('k') ,
            0x26 => KeyCode::L('l') ,
            0x27 => KeyCode::Semicolon(';') ,
            0x28 => KeyCode::Apostrophe('\'') ,
            0x29 => KeyCode::Grave('`') ,
            0x2A => KeyCode::ShiftLeft ,
            0x2B => KeyCode::Backslash('\\') ,
            0x2C => KeyCode::Z('z') ,
            0x2D => KeyCode::X('x') ,
            0x2E => KeyCode::C('c') ,
            0x2F => KeyCode::V('v') ,
            0x30 => KeyCode::B('b') ,
            0x31 => KeyCode::N('n') ,
            0x32 => KeyCode::M('m') ,
            0x33 => KeyCode::Comma(',') ,
            0x34 => KeyCode::Period('.') ,
            0x35 => KeyCode::Forwardslash('/') ,
            0x36 => KeyCode::ShiftRight ,
            0x37 => KeyCode::KeypadAsterisk('*') ,
            0x38 => KeyCode::AltLeft ,
            0x39 => KeyCode::Space(' ') ,
            0x3A => KeyCode::CapsLock ,
            0x3B => KeyCode::F1 ,
            0x3C => KeyCode::F2 ,
            0x3D => KeyCode::F3 ,
            0x3E => KeyCode::F4 ,
            0x3F => KeyCode::F5 ,
            0x40 => KeyCode::F6 ,
            0x41 => KeyCode::F7 ,
            0x42 => KeyCode::F8 ,
            0x43 => KeyCode::F9,
            0x44 => KeyCode::F10 ,
            0x45 => KeyCode::NumLock ,
            0x46 => KeyCode::ScrollLock ,
            0x47 => KeyCode::Keypad7(7) ,
            0x48 => KeyCode::Keypad8(8) ,
            0x49 => KeyCode::Keypad9(9) ,
            0x4A => KeyCode::KeypadHyphen('-') ,
            0x4B => KeyCode::Keypad4(4) ,
            0x4C => KeyCode::Keypad5(5) ,
            0x4D => KeyCode::Keypad6(6) ,
            0x4E => KeyCode::KeypadPlus('+') ,
            0x4F => KeyCode::Keypad1(1) ,
            0x50 => KeyCode::Keypad2(2) ,
            0x51 => KeyCode::Keypad3(3),
            0x52 => KeyCode::Keypad0(0),
            0x53 => KeyCode::KeypadPeriod('.'),
            // ...
            0x57 => KeyCode::F11,
            0x58 => KeyCode::F12,

            _ => KeyCode::None        
        }
    }
}

// impl Index<u8> for Ps2Controller(

// )