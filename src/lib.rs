use mio::Token;
use std::fmt;


#[derive(Debug)]
pub struct PacketData {
    pub token: Token,
    pub data: Vec<u8>,
}
impl fmt::Display for PacketData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{token:{:?}, cmd:{}, length:{}}}",
            self.token,
            self.data.first().unwrap(),
            self.data.len()
        )
    }
}


#[allow(dead_code, non_snake_case)]
pub mod ANSI {
    pub const RESET: &str = "\x1b[0m";
}


#[allow(dead_code, non_snake_case)]
pub const INTERNAL_OPCODE: u8 = b'\xFE';


pub enum InternalOpcodeInstruction {
    None,
    SetPlayerState
}
impl From<u8> for InternalOpcodeInstruction {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => return InternalOpcodeInstruction::None,
            0x1 => return InternalOpcodeInstruction::SetPlayerState,
            _ => return InternalOpcodeInstruction::None
        };
    }
}


#[allow(dead_code, non_snake_case)]
pub mod TELNET {
    pub const IAC: u8 = b'\xFF';
}


#[allow(dead_code)]
pub struct Players {
    pub state: [PlayerState; 4096]
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum PlayerState {
    None,
    LoggingIn,
    CharacterCreation
}
impl From<u8> for PlayerState {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => return PlayerState::None,
            0x1 => return PlayerState::LoggingIn,
            0x2 => return PlayerState::CharacterCreation,
            _ => return PlayerState::None
        };
    }
}