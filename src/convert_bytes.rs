use crate::structures::{ParsingError::Empty, TrackedError};

pub fn transform(input: Vec<u32>) -> Result<Vec<u8>, TrackedError> {
    if input.is_empty() {
        return Err(TrackedError {
            kind: Empty,
            line: line!(),
            file: file!(),
        });
    }
    let mut bytes: Vec<u8> = Vec::new();
    for word in input {
        bytes.extend_from_slice(&word.to_le_bytes());
    }

    Ok(bytes)
}
