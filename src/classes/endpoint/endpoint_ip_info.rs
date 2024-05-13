use crate::classes::addresses::mac_address::MacAddress;

pub struct EndpointIPInformation {
    pub device_id: u64,
    pub endpoint_id: u64,
    pub device_mac: MacAddress,
    pub endpoint_address: MacAddress,
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