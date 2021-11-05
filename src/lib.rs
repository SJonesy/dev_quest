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
