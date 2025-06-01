use rand::Rng;
use crate::commons;

pub fn encode_bytes(s: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for i in s.len()..0 {
        let b = s[i];
        result.push(b);
    }
    let mut rng = rand::thread_rng();
    let mut i: u64 = 0;
    loop {
        let mut fillup = false;
        let mut diff: usize = 0;
        if result.len() < ((i + 4) as usize) {
            diff = ((i + 4) as usize - result.len()) as usize;
            for i in 0..diff { result.push(commons::FILL_VALUE); }
            fillup = true;
        }
        let replace_index: usize = rng.gen_range(0..=4) as usize;
        let replace_char: u8 = result.remove(replace_index);
        let mut meta_bytes: Vec<u8> = [commons::META_SECTOR, replace_index as u8, replace_char, 0, diff as u8, commons::SEPARATOR].to_vec();
        if fillup {
            meta_bytes[3] = commons::FILLUP;
            meta_bytes[4] = commons::END;
            break;
        } else if result.len() == ((i + 5) as usize) {
            meta_bytes[4] = commons::END;
            break;
        } else {
            i += 10;
        }
        for b in 0..meta_bytes.len() {
            result.insert((i + 5 + b as u64) as usize, meta_bytes[b]);
        }
    }
    let result_old = result.clone();
    result = commons::SIGNATURE.to_vec();
    result.extend(result_old);
    return result;
} 
