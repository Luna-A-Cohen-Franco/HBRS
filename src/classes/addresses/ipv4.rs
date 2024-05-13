use crate::consts::errors::HbrsError;

pub struct IPv4Addr {
    octets: Vec<u8>,
}

impl IPv4Addr {
    fn new(octets: Vec<u8>) -> Result<Self, HbrsError> {
        if octets.len() != 4 {
            return Err(HbrsError::BadIpv4AddressLength);
        }
        Ok(Self { octets })
    }
}