use crate::classes::addresses::mac_address::MacAddress;
use crate::consts::errors::HbrsError;
use crate::consts::other::Other;

#[derive(Debug)]
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

    pub fn clear_bytes(&mut self){
        self.protocol_version = 0;
        self.source_mac = MacAddress::new_empty();
        self.destination_mac = MacAddress::new_empty();
        self.sequence_number = 0;
        self.source_endpoint = 0;
        self.destination_endpoint = 0;
        self.command_id = 0;
        self.mac_of_last_response = MacAddress::new_empty();
    }

    pub fn get_id(&self) -> u8{
        return self.command_id;
    }

    pub fn set_id(&mut self, id: u8){
        self.command_id = id;
    }

    pub fn get_source_endpoint(&self) -> u8{
        return self.source_endpoint;
    }

    pub fn set_source_endpoint(&mut self, endpoint: u8){
        self.source_endpoint = endpoint;
    }

    pub fn get_destination_endpoint(&self) -> u8{
        return self.destination_endpoint;
    }

    pub fn set_destination_endpoint(&mut self, endpoint: u8){
        self.destination_endpoint = endpoint;
    }

    pub fn get_sequence_number(&self) -> u8{
        return self.sequence_number;
    }

    pub fn set_sequence_number(&mut self, sequence_number: u8){
        self.sequence_number = sequence_number;
    }

    pub fn get_source_mac(&self) -> MacAddress{
        return self.source_mac.clone();
    }

    pub fn set_source_mac(&mut self, mac: MacAddress){
        self.source_mac = mac;
    }

    pub fn get_destination_mac(&self) -> MacAddress{
        return self.destination_mac.clone();
    }

    pub fn set_destination_mac(&mut self, mac: MacAddress){
        self.destination_mac = mac;
    }

    pub fn get_protocol_version(&self) -> u8{
        return self.protocol_version;
    }

    pub fn set_protocol_version(&mut self, version: u8){
        self.protocol_version = version;
    }

    pub fn get_mac_of_last_response(&self) -> MacAddress{
        return self.mac_of_last_response.clone();
    }

    pub fn set_mac_of_last_response(&mut self, mac: MacAddress){
        self.mac_of_last_response = mac;
    }
}