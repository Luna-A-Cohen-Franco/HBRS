use crate::classes::addresses::mac_address::MacAddress;
use crate::consts::errors::HbrsError;
use crate::consts::other::Other;

pub struct Header{
    protocol_version: u8,
    source_mac: MacAddress,
    destination_mac: MacAddress,
    sequence_number: u8,
    source_endpoint: u8,
    destination_endpoint: u8,
    command_id: u8,
    mac_of_last_response: MacAddress,
}

impl Header{
    pub fn new() -> Self{
        Self{
            protocol_version: 0,
            source_mac: MacAddress::new_empty(),
            destination_mac: MacAddress::new_empty(),
            sequence_number: 0,
            source_endpoint: 0,
            destination_endpoint: 0,
            command_id: 0,
            mac_of_last_response: MacAddress::new_empty(),
        }
    }
    pub fn get_bytes(&self) -> Vec<u8>{
        let mut bytes: Vec<u8> = Vec::new();

        for _ in 0..Other::HeaderOffset.get_value(){
            bytes.push(0);
        }

        let mut index = 0usize;

        bytes[index] = self.protocol_version;
        index += 1;

        for byte in self.source_mac.get_bytes(){
            bytes[index] = *byte;
            index += 1;
        }

        index = 7;

        for byte in self.destination_mac.get_bytes(){
            bytes[index] = *byte;
            index += 1;
        }
        
        index = 13;
        bytes[index] = self.sequence_number;
        index += 1;
        bytes[index] = self.source_endpoint;
        index += 1;
        bytes[index] = self.destination_endpoint;
        index += 1;
        bytes[index] = self.command_id;
        
        return bytes;
    }

    pub fn set_bytes(&mut self, data: Vec<u8>) -> Result<bool, HbrsError>{
        let mut index = 0usize;

        self.protocol_version = data[index];
        index += 1;

        self.mac_of_last_response = self.source_mac.clone();
        let source_mac_bytes = data[index..index + 6].to_vec();
        self.source_mac = match MacAddress::new(source_mac_bytes){
            Ok(mac) => mac,
            Err(e) => return Err(e),
        };

        index = 7;
        let destination_mac_bytes = data[index..index + 6].to_vec();
        self.destination_mac = match MacAddress::new(destination_mac_bytes){
            Ok(mac) => mac,
            Err(e) => return Err(e),
        };

        index = 13;
        self.sequence_number = data[index];
        index += 1;
        self.source_endpoint = data[index];
        index += 1;
        self.destination_endpoint = data[index];
        index += 1;
        self.command_id = data[index];

        return Ok(true);
    }
}