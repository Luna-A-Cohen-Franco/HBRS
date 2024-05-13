use crate::consts::errors::HbrsError;

#[derive(Debug, Clone)]
pub struct IPv4Addr {
    octets: Vec<u8>,
}

impl IPv4Addr {
    pub fn new(octets: Vec<u8>) -> Result<Self, HbrsError> {
        if octets.len() != 4 {
            return Err(HbrsError::BadIpv4AddressLength);
        }
        Ok(Self { octets })
    }

    pub fn get_bytes_ref(&self) -> &Vec<u8> {
        return &self.octets;
    }

    pub fn get_bytes_mut(&mut self) -> &mut Vec<u8> {
        return &mut self.octets;
    }
}