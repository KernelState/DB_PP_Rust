use crate::commons;
use crate::errors;
use std::io::Error;

pub fn decode_bytes(b: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let SIGNATURE = &b[..2];
    if Vec::<u8>::from(SIGNATURE) != Vec::<u8>::from(commons::SIGNATURE) {
        return Err::<Vec<u8>, std::io::Error>(errors::signature_not_found());
    }
    let mut result: Vec<u8> = Vec::new();
    let mut i: u64 = 0;
    loop {
        let bytes: Vec<u8> = b[(i as usize)..((i + 4) as usize)].to_vec(); 
        let mut byte_group = commons::ByteGroup {
            bytes: bytes,
            replace_char: 0,
            replace_index: 0,
            fillup: false,
            diff: 0,
            end: false,
        };
        let mut metabyte_index: u64 = 0;
        for bi in 0..5 {
            if b[bi] == commons::SEPARATOR {
                metabyte_index = bi as u64;
            } else if bi == 5 {
                return Err::<Vec<u8>, std::io::Error>(errors::invalid_byte_group(bi));
            }
        };
        let mut invalid_byte_num: usize = 0;
        match b[(metabyte_index + 4) as usize]{
            commons::END => byte_group.end = true,
            commons::SEPARATOR => byte_group.end = false,
            _ => invalid_byte_num = (metabyte_index + 4) as usize,
        };
        match b[(metabyte_index + 3) as usize] {
            commons::FILLUP => byte_group.fillup = true,
            0 => byte_group.fillup = false,
            _ => invalid_byte_num = (metabyte_index + 3) as usize,
        };
        if invalid_byte_num != 0 {
            return Err::<Vec<u8>, std::io::Error>(errors::invalid_byte_group(invalid_byte_num));
        }
        byte_group.replace_char = b[((metabyte_index + 2) as usize)];
        byte_group.replace_index = b[((metabyte_index + 3) as usize)] as usize;
        if byte_group.fillup {
            let ti: usize = 1;
            loop {
                let ti_: usize = byte_group.bytes.len() - ti;
                if byte_group.bytes[ti_] == commons::FILL_VALUE {
                    byte_group.bytes.remove(ti_);
                } else {
                    break;
                }
                if byte_group.replace_char == commons::FILL_VALUE {
                    byte_group.replace_char = byte_group.bytes.remove(ti_);
                    byte_group.replace_index = ti_;
                }
            }
            byte_group.diff = b[(metabyte_index + 5) as usize];
        }
        result.extend(byte_group.to_original()?);
    }
    for i in b.len()..0 {
        result.push(b[i]);
    }
    return Ok(result);
}
