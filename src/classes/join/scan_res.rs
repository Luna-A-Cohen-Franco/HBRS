use super::scan_res_item::ScanResItem;
use crate::consts::{errors::HbrsError, response_item::ResponseItem::{DataSize, MinRSSI}};
pub struct ScanRes{
    pub wifis: Vec<ScanResItem>,
}

impl ScanRes{
    pub fn new() -> Self{
        Self{
            wifis: Vec::new(),
        }
    }

    pub fn add_new_item(&mut self, data: Vec<u8>, header_offset: usize) -> Result<bool, HbrsError>{
        if data.len() != DataSize.get_value(){
            return Err(HbrsError::InvalidScanDataLength)
        }

        let new_item = match ScanResItem::new(data, header_offset){
            Ok(item) => item,
            Err(err) => return Err(err),
        };

        // Checks validity of SSID
        if new_item.ssid.iter().all(|&val| val == 0) 
            || new_item.get_ssid_as_str().is_err() 
            || new_item.rssi <= MinRSSI.get_value() as u8{
            return Err(HbrsError::BadSSID);
        }

        // If the item already exists, update the RSSI value
        let existing_item = 
            self.wifis.iter_mut()
                .find( |i|
                i.get_ssid_as_str().unwrap().to_lowercase() 
                == new_item.get_ssid_as_str().unwrap().to_lowercase());
        
        match existing_item{
            Some(existing_item) => {
                if existing_item.rssi < new_item.rssi{
                    existing_item.rssi = new_item.rssi;
                }
            },
            None => {},
        }

        self.wifis.push(new_item);

        return Ok(true);
    }
}

