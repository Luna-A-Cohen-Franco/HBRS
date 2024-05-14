use crate::utils::byte_arrays_helper::ByteArraysHelper;

#[derive(Debug, Clone)]
pub struct Ping{
    length: u8,
    data: Vec<u8>,
}

impl Ping{
    pub fn new() -> Self{
        Self{
            length: 1,
            data: vec![65],
        }
    }

    pub fn get_bytes(&self) -> Vec<u8>{
       ByteArraysHelper::combine_1v_1b(&self.data, self.length)
    }

    pub fn get_length_ref(&self) -> &u8 {
        &self.length
    }
    
    pub fn get_length_mut(&mut self) -> &mut u8 {
        &mut self.length
    }
    
    pub fn get_data_ref(&self) -> &Vec<u8> {
        &self.data
    }
    
    pub fn get_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}