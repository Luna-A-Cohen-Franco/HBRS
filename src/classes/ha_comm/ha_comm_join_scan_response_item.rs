use std::string::FromUtf8Error;

use crate::consts::{other::OtherConsts::KeyLength, response_item::ResponseItem};

pub struct ha_comm_join_scan_response_item{
    pub ssid: Vec<u8>,
    pub security_type: u8,
    pub encryption_type: u8,
    pub rssi: u8,
}

impl ha_comm_join_scan_response_item{
    pub fn new(data: Vec<u8>, header_offset: usize) -> Self{
        let mut ssid = vec![0; KeyLength.get_value()];
        
        ssid.copy_from_slice(&data[header_offset..(header_offset+32)]);

		let security_type = data[header_offset + ResponseItem::SecurityTypePos.get_value()];
        let encryption_type = data[header_offset + ResponseItem::EncryptionTypePos.get_value()];
		let rssi = data[header_offset + ResponseItem::RSSIPos.get_value()];
        
        Self{
            ssid,
            security_type,
            encryption_type,
            rssi,
        }
    }

    pub fn get_ssid_as_str(&self) -> Result<String, FromUtf8Error>{
        return String::from_utf8(self.ssid.clone());
    }
}