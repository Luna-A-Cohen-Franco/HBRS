use std::string::FromUtf8Error;

use crate::consts::{errors::HbrsError, other::Other::KeyLength, response_item::ResponseItem};

#[derive(Debug, Clone)]
pub struct ScanResItem{
    pub ssid: Vec<u8>,
    pub security_type: u8,
    pub encryption_type: u8,
    pub rssi: u8,
}

impl ScanResItem{
    pub fn new(data: Vec<u8>, header_offset: usize) -> Result<Self, HbrsError>{
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

    pub fn get_rssi(&self) -> u8{
        return self.rssi;
    }

    pub fn set_rssi(&mut self, rssi: u8){
        self.rssi = rssi;
    }
}