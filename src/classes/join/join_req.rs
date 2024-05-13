use crate::{consts::{other::Other, errors::HbrsError}, utils::byte_arrays_helper::ByteArraysHelper};

#[derive(Debug, Clone)]
pub struct JoinReq{
    pub ssid: Vec<u8>,
    pub security_type: u8,
    pub encryption_type: u8,
    pub key: Vec<u8>,
}

impl JoinReq{
    pub fn new(ssid: Vec<u8>, security_type: u8, encryption_type: u8, key: Vec<u8>) -> Result<Self, HbrsError>{
        let key = match ByteArraysHelper::cp_arr_bytes_fill(&key, Other::KeyLength.get_value()){
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        Ok(Self{
            ssid: ssid,
            security_type: security_type,
            encryption_type: encryption_type,
            key: key,
        })
    }

    pub fn get_bytes(&self) -> Vec<u8>{
        let first = ByteArraysHelper::combine_1v_1b(&self.ssid, self.security_type);
        let second = ByteArraysHelper::combine_1v_1b(&self.key, self.encryption_type);
        return ByteArraysHelper::combine_2v(&first, &second);
    }

    pub fn get_ssid(&self) -> Vec<u8>{
        return self.ssid.clone();
    }

    pub fn set_ssid(&mut self, ssid: Vec<u8>){
        self.ssid = ssid;
    }

    pub fn get_security_type(&self) -> u8{
        return self.security_type;
    }

    pub fn set_security_type(&mut self, security_type: u8){
        self.security_type = security_type;
    }

    pub fn get_encryption_type(&self) -> u8{
        return self.encryption_type;
    }

    pub fn set_encryption_type(&mut self, encryption_type: u8){
        self.encryption_type = encryption_type;
    }

    pub fn get_key(&self) -> Vec<u8>{
        return self.key.clone();
    }

    pub fn set_key(&mut self, key: Vec<u8>) -> Result<(), HbrsError>{
        self.key = match ByteArraysHelper::cp_arr_bytes_fill(&key, Other::KeyLength.get_value()){
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        Ok(())
    }
}