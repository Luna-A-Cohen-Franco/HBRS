use crate::classes::addresses::mac_address::MacAddress;
use crate::consts::errors::HbrsError;
use crate::consts::other::Other;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
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

        for byte in self.source_mac.get_bytes_ref(){
            bytes[index] = *byte;
            index += 1;
        }

        index = 7;

        for byte in self.destination_mac.get_bytes_ref(){
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

    pub fn set_bytes(&mut self, data: &Vec<u8>) -> Result<bool, HbrsError>{
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

    pub fn get_id_ref(&self) -> &u8{
        return &self.command_id;
    }

    pub fn get_id_mut(&mut self) -> &mut u8{
        return &mut self.command_id;
    }

    pub fn get_source_endpoint_ref(&self) -> &u8 {
        &self.source_endpoint
    }
    
    pub fn get_source_endpoint_mut(&mut self) -> &mut u8 {
        &mut self.source_endpoint
    }
    
    pub fn get_destination_endpoint_ref(&self) -> &u8 {
        &self.destination_endpoint
    }
    
    pub fn get_destination_endpoint_mut(&mut self) -> &mut u8 {
        &mut self.destination_endpoint
    }
    
    pub fn get_sequence_number_ref(&self) -> &u8 {
        &self.sequence_number
    }
    
    pub fn get_sequence_number_mut(&mut self) -> &mut u8 {
        &mut self.sequence_number
    }
    
    pub fn get_source_mac_ref(&self) -> &MacAddress {
        &self.source_mac
    }
    
    pub fn get_source_mac_mut(&mut self) -> &mut MacAddress {
        &mut self.source_mac
    }
    
    pub fn get_destination_mac_ref(&self) -> &MacAddress {
        &self.destination_mac
    }
    
    pub fn get_destination_mac_mut(&mut self) -> &mut MacAddress {
        &mut self.destination_mac
    }
    
    pub fn get_protocol_version_ref(&self) -> &u8 {
        &self.protocol_version
    }
    
    pub fn get_protocol_version_mut(&mut self) -> &mut u8 {
        &mut self.protocol_version
    }
    
    pub fn get_mac_of_last_response_ref(&self) -> &MacAddress {
        &self.mac_of_last_response
    }
    
    pub fn get_mac_of_last_response_mut(&mut self) -> &mut MacAddress {
        &mut self.mac_of_last_response
    }
}