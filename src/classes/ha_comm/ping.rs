use crate::utils::byte_arrays_helper::ByteArraysHelper;

#[derive(Debug)]
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

    pub fn set_data(&mut self, data: Vec<u8>){
        self.data = data;
    }
}