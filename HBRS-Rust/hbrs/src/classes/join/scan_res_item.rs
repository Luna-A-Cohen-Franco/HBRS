use std::string::FromUtf8Error;
use serde::{Serialize, Deserialize};

use crate::consts::{errors::HbrsError, other::Other::KeyLength, response_item::ResponseItem};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct ScanResItem{
    ssid: Vec<u8>,
    security_type: u8,
    encryption_type: u8,
    rssi: u8,
}

impl ScanResItem{
    pub fn new(data: &Vec<u8>, header_offset: usize) -> Result<Self, HbrsError>{
		if data.len() < ResponseItem::DataSize.get_value(){
			return Err(HbrsError::InvalidScanDataLength);
		}

        let mut ssid = vec![0; KeyLength.get_value()];
        
        ssid.copy_from_slice(&data[header_offset..(header_offset+32)]);

		let security_type = data[header_offset + ResponseItem::SecurityTypePos.get_value()];
        let encryption_type = data[header_offset + ResponseItem::EncryptionTypePos.get_value()];
		let rssi = data[header_offset + ResponseItem::RSSIPos.get_value()];
        
        Ok(Self{
            ssid,
            security_type,
            encryption_type,
            rssi,
        })
    }

    pub fn get_ssid_as_str(&self) -> Result<String, FromUtf8Error>{
        return String::from_utf8(self.ssid.clone());
    }

    pub fn get_bytes(&self) -> Vec<u8>{
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.ssid);
        bytes.push(self.security_type);
        bytes.push(self.encryption_type);
        bytes.push(self.rssi);

        return bytes;
    }

    pub fn get_ssid_ref(&self) -> &Vec<u8> {
        &self.ssid
    }
    
    pub fn get_ssid_mut(&mut self) -> &mut Vec<u8> {
        &mut self.ssid
    }
    
    pub fn get_security_type_ref(&self) -> &u8 {
        &self.security_type
    }
    
    pub fn get_security_type_mut(&mut self) -> &mut u8 {
        &mut self.security_type
    }
    
    pub fn get_encryption_type_ref(&self) -> &u8 {
        &self.encryption_type
    }
    
    pub fn get_encryption_type_mut(&mut self) -> &mut u8 {
        &mut self.encryption_type
    }
    
    pub fn get_rssi_ref(&self) -> &u8 {
        &self.rssi
    }
    
    pub fn get_rssi_mut(&mut self) -> &mut u8 {
        &mut self.rssi
    }
}