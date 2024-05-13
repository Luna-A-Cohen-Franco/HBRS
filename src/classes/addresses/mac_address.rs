use crate::consts::errors::HbrsError;

#[derive(Debug, Clone)]
pub struct MacAddress {
    bytes: Vec<u8>,
}

impl MacAddress {
    pub fn new(bytes: Vec<u8>) -> Result<Self, HbrsError> {
        if bytes.len() != 6 {
            return Err(HbrsError::BadMacAddressLength);
        }

        return Ok(Self { bytes });
    }
    pub fn new_empty() -> Self {
        return Self { bytes: Vec::new() };
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn display(&self) {
        for byte in &self.bytes {
            print!("{:02X}:", byte);
        }
        println!();
    }

    pub fn from_string(addr: String) -> Result<MacAddress, HbrsError>{
        if addr.len() != 12{
            return Err(HbrsError::BadMacAddressLength);
        }

        for c in addr.chars(){
            if !c.is_digit(16){
                return Err(HbrsError::BadMacAddressValues);
            }
        }

        let mut bytes: Vec<u8> = Vec::new();

        for i in (0..addr.len()).step_by(2){
            let byte = u8::from_str_radix(&addr[i..i+2], 16).unwrap();
            bytes.push(byte);
        }

        return MacAddress::new(bytes);
    }

    pub fn get_bytes_mut(&mut self) -> &mut Vec<u8> {
        return &mut self.bytes;
    }

    pub fn get_bytes_ref(&self) -> &Vec<u8> {
        return &self.bytes;
    }
}