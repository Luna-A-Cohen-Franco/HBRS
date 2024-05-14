use crate::{consts::errors::HbrsError, utils::byte_arrays_helper::ByteArraysHelper};

#[derive(Debug, Clone)]
pub struct CloudNotifIP{
    address: Vec<u8>,
}
impl CloudNotifIP{
    pub fn new(address: Vec<u8>) -> Result<Self, HbrsError>{
        let address: Vec<u8> = match ByteArraysHelper::cp_arr_bytes_fill(&address, 16){
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Ok(Self{
            address
        })
    }
    pub fn get_bytes(&self) -> Vec<u8>{
        return self.address.clone();
    }

    pub fn set_bytes(&mut self, address: Vec<u8>) -> Result<(), HbrsError>{
        self.address = match ByteArraysHelper::cp_arr_bytes_fill(&address, 16){
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    pub fn get_address_ref(&self) -> &Vec<u8>{
        return &self.address;
    }

    pub fn get_address_mut(&mut self) -> &mut Vec<u8>{
        return &mut self.address;
    }
}