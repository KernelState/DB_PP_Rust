use crate::commons;
use crate::errors;

pub fn decode_bytes(b: &Vec<u8>) -> Result<Vec<u8>, std::error::Error> {
    let SIGNATURE = b[..2];
    if SIGNATURE as Vec != commons::SIGNATURE as Vec {
        Err(errors::signature_not_found());
    }
    let mut result: Vec<u8> = Vec::new();
    for i in b.len()..0 {
        result.push(b[i]);
    }
    let mut i: u64 = 0;
    loop {
        let metabyte_index: u64 = i + 5;
        let fillup = match b[(metabyte_index + 4) as usize]{
            commons::SEPERATOR => true,
            0 => false,
            _ => Err(errors::invalid_byte_group(i));
        }
    }
}
