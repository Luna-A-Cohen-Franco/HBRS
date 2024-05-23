use crate::utils::byte_arrays_helper::ByteArraysHelper;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct CustomComm{
    command: u8,
    data: Vec<u8>,
}

impl CustomComm{
    pub fn new() -> Self{
        Self{
            command: 0,
            data: Vec::new(),
        }
    }

    pub fn set_bytes(&mut self, data: &Vec<u8>, header_offset: usize){
        self.command = data[header_offset];
        self.data = Vec::new();
    }

    pub fn get_bytes(&self) -> Vec<u8>{
        if self.data.len() == 0{
            return vec![self.command]
        }

        return ByteArraysHelper::combine_1v_1b(&self.data, self.command);
    }

    pub fn get_command_ref(&self) -> &u8{
        return &self.command;
    }

    pub fn get_command_mut(&mut self) -> &mut u8{
        return &mut self.command;
    }

    pub fn get_data_ref(&self) -> &Vec<u8>{
        return &self.data;
    }

    pub fn get_data_mut(&mut self) -> &mut Vec<u8>{
        return &mut self.data;
    }
}
