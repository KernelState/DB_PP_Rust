use std::io::{Error, ErrorKind};

pub fn signature_not_found() -> Error {
    return Error::new(ErrorKind::InvalidData, "InvalidData: the inputed value did not include the signature.");
}

pub fn invalid_byte_group(group_index: usize) -> Error {
    return Error::new(ErrorKind::Unsupported, "unsupported group structure from index `" + group_index as &str + "` please make sure you used the right version of this module/crate to encrypt.");
}
