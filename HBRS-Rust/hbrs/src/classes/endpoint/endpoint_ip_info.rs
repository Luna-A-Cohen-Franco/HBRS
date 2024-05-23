use crate::classes::addresses::mac_address::MacAddress;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct EndpointIPInformation {
    device_id: u64,
    endpoint_id: u64,
    device_mac: MacAddress,
    endpoint_address: MacAddress,
}

impl EndpointIPInformation {
    pub fn new(device_id: u64, endpoint_id: u64, device_mac: MacAddress, endpoint_address: MacAddress) -> Self {
        Self {
            device_id,
            endpoint_id,
            device_mac,
            endpoint_address,
        }
    }
  
   pub fn get_device_id_ref(&self) -> &u64 {
       return &self.device_id;
   }

    pub fn get_device_id_mut(&mut self) -> &mut u64 {
         return &mut self.device_id;
    }

    pub fn get_endpoint_id_ref(&self) -> &u64 {
        return &self.endpoint_id;
    }

    pub fn get_endpoint_id_mut(&mut self) -> &mut u64 {
        return &mut self.endpoint_id;
    }

    pub fn get_device_mac_ref(&self) -> &MacAddress {
        return &self.device_mac;
    }

    pub fn get_device_mac_mut(&mut self) -> &mut MacAddress {
        return &mut self.device_mac;
    }

    pub fn get_endpoint_address_ref(&self) -> &MacAddress {
        return &self.endpoint_address;
    }

    pub fn get_endpoint_address_mut(&mut self) -> &mut MacAddress {
        return &mut self.endpoint_address;
    }
}
// Still up in the air as to what type of address csharp code is refering to 
