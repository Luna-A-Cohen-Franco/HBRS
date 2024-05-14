use crate::consts::errors::HbrsError;

#[derive(Debug, Clone)]
pub struct IPv4Addr {
    bytes: Vec<u8>,
}

impl IPv4Addr {
    pub fn new(bytes: Vec<u8>) -> Result<Self, HbrsError> {
        if bytes.len() != 4 {
            return Err(HbrsError::BadIpv4AddressLength);
        }
        Ok(Self{ bytes })
    }

    pub fn get_bytes_ref(&self) -> &Vec<u8> {
        return &self.bytes;
    }

    pub fn get_bytes_mut(&mut self) -> &mut Vec<u8> {
        return &mut self.bytes;
    }
}