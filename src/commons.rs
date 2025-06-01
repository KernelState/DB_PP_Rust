pub use std::io::{Error, ErrorKind};
use crate::errors;

pub const META_SECTOR: u8 = 0xFF;
pub const END: u8 = 0xF5;
pub const SEPARATOR: u8 = 0xC0;
pub const FILLUP: u8 = 0xC1;
pub const SIGNATURE: [u8; 3] = [0xF5, 0xFD, 0xF7];
pub const FILL_VALUE: u8 = 0x80;

#[derive(Debug)]
pub struct ByteGroup {
    pub bytes: Vec<u8>,
    pub replace_char: u8,
    pub replace_index: usize,
    pub fillup: bool,
    pub diff: u8,
    pub end: bool,
}

impl ByteGroup {
    pub fn to_original(&self) -> Result<Vec<u8>, Error> {
        let mut result: Vec<u8> = self.bytes.to_vec();
        if self.replace_index == 0 || self.replace_char == 0 {
            return Err::<_, std::io::Error>(errors::invalid_byte_group(0));
        }
        result.insert(self.replace_index, self.replace_char);
        return Ok(result);
    }
}
