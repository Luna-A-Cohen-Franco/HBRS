use super::addresses::{ipv4::IPv4Addr, mac_address::MacAddress};

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
}