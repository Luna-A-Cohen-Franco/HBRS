use super::addresses::{ipv4::IPv4Addr, mac_address::MacAddress};

#[derive(Debug, Clone)]
pub struct DeviceIPInfo
{
   device_id: i64,
   device_mac: MacAddress,
   local_ip_address: IPv4Addr
}

impl DeviceIPInfo{
    pub fn new(device_id: i64, device_mac: MacAddress, local_ip_address: IPv4Addr) -> Self{
        Self{
            device_id,
            device_mac,
            local_ip_address,
        }
    }

    pub fn get_device_id_ref(&self) -> &i64 {
        &self.device_id
    }
    
    pub fn get_device_id_mut(&mut self) -> &mut i64 {
        &mut self.device_id
    }
    
    pub fn get_device_mac_ref(&self) -> &MacAddress {
        &self.device_mac
    }
    
    pub fn get_device_mac_mut(&mut self) -> &mut MacAddress {
        &mut self.device_mac
    }
    
    pub fn get_local_ip_address_ref(&self) -> &IPv4Addr {
        &self.local_ip_address
    }
    
    pub fn get_local_ip_address_mut(&mut self) -> &mut IPv4Addr {
        &mut self.local_ip_address
    }
}