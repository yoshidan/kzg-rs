use crate::enums::KzgError;
use crate::{BYTES_PER_BLOB, BYTES_PER_FIELD_ELEMENT};

use alloc::{string::ToString, vec::Vec};

#[derive(Debug, Clone)]
pub struct Blob([u8; BYTES_PER_BLOB]);

impl Blob {
    pub fn from_slice(slice: &[u8]) -> Result<Self, KzgError> {
        if slice.len() != BYTES_PER_BLOB {
            return Err(KzgError::InvalidBytesLength(
                "Invalid slice length".to_string(),
            ));
        }
        let mut bytes = [0u8; BYTES_PER_BLOB];
        bytes.copy_from_slice(slice);
        Ok(Blob(bytes))
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl From<Blob> for [u8; BYTES_PER_BLOB] {
    fn from(value: Blob) -> [u8; BYTES_PER_BLOB] {
        value.0
    }
}

#[derive(Debug, Clone)]
pub struct Bytes32([u8; 32]);

impl Bytes32{
    pub fn from_slice(slice: &[u8]) -> Result<Self, KzgError> {
        if slice.len() != 32 {
            return Err(KzgError::InvalidBytesLength(
                "Invalid slice length".to_string(),
            ));
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(slice);
        Ok(Bytes32(bytes))
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl From<Bytes32> for [u8; 32] {
    fn from(value: Bytes32) -> [u8; 32] {
        value.0
    }
}


#[derive(Debug, Clone)]
pub struct Bytes48([u8; 48]);

impl Bytes48{
    pub fn from_slice(slice: &[u8]) -> Result<Self, KzgError> {
        if slice.len() != 48 {
            return Err(KzgError::InvalidBytesLength(
                "Invalid slice length".to_string(),
            ));
        }
        let mut bytes = [0u8; 48];
        bytes.copy_from_slice(slice);
        Ok(Bytes48(bytes))
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl From<Bytes48> for [u8; 48] {
    fn from(value: Bytes48) -> [u8; 48] {
        value.0
    }
}
