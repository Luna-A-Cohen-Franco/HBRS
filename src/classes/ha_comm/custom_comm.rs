use crate::utils::byte_arrays_helper::ByteArraysHelper;

pub struct CustomComm{
    pub command: u8,
    pub data: Vec<u8>,
}

impl CustomComm{
    pub fn new() -> Self{
        Self{
            command: 0,
            data: Vec::new(),
        }
    }

    pub fn set_bytes(&mut self, data: Vec<u8>, header_offset: usize){
        self.command = data[header_offset];
        self.data = Vec::new();
    }

    pub fn get_byte(&self) -> Vec<u8>{
        if self.data.len() == 0{
            return vec![self.command]
        }

        return ByteArraysHelper::combine_1v_1b(&self.data, self.command);
    }
}
