use super::addresses::{ipv4::IPv4Addr, mac_address::MacAddress};

#[derive(Debug, Clone)]
pub struct DeviceIPInfo
{
   pub device_id: i64,
   pub device_mac: MacAddress,
   pub local_ip_address: IPv4Addr
}

impl DeviceIPInfo{
    pub fn new(device_id: i64, device_mac: MacAddress, local_ip_address: IPv4Addr) -> Self{
        Self{
            device_id,
            device_mac,
            local_ip_address,
        }
    }

    pub fn get_device_id(&self) -> i64{
        self.device_id
    }

    pub fn get_device_mac(&self) -> MacAddress{
        self.device_mac.clone()
    }

    pub fn get_local_ip_address(&self) -> IPv4Addr{
        self.local_ip_address.clone()
    }

    pub fn set_device_id(&mut self, device_id: i64){
        self.device_id = device_id;
    }

    pub fn set_device_mac(&mut self, device_mac: MacAddress){
        self.device_mac = device_mac;
    }

    pub fn set_local_ip_address(&mut self, local_ip_address: IPv4Addr){
        self.local_ip_address = local_ip_address;
    }
}