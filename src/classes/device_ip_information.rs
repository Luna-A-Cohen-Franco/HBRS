use super::{ipv4::IPv4Addr, mac_address::MacAddress};

pub struct DeviceIPInformation
{
   pub DeviceID: i64,
   pub DeviceMac: MacAddress,
   pub LocalIPAddress: IPv4Addr
}

impl DeviceIPInformation{
    pub fn new(DeviceID: i64, DeviceMac: MacAddress, LocalIPAddress: IPv4Addr) -> DeviceIPInformation{
        DeviceIPInformation{
            DeviceID,
            DeviceMac,
            LocalIPAddress,
        }
    }
}