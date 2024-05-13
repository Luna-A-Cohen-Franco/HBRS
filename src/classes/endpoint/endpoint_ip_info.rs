use crate::classes::addresses::mac_address::MacAddress;

#[derive(Debug, Clone)]
pub struct EndpointIPInformation {
    pub device_id: u64,
    pub endpoint_id: u64,
    pub device_mac: MacAddress,
    pub endpoint_address: MacAddress,
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
// Write get ref and get mut for all attributes
   
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
// Still up in the air as to what type of address this is refering to 

/*
    public class EndpointIPInformation
    {
        public long DeviceID { get; set; }

        public long EndpointID { get; set; }

        public string DeviceMac { get; set; }
        
        public string EndpointAddress { get; set; }
    }
*/