use crate::utils::byte_arrays_helper::ByteArraysHelper;

#[derive(Debug)]
pub struct SetAutoOff{
    off_timeout: u8,
}

impl SetAutoOff{
    pub fn new() -> Self{
        Self{
            off_timeout: 0,
        }
    }

    pub fn get_bytes(&self) -> Vec<u8>{
        let (b1, b2) = ByteArraysHelper::word_to_port16(self.off_timeout as u16);
        
        return ByteArraysHelper::combine_2b(b1, b2);
    }

    pub fn get_off_timeout(&self) -> u8{
        return self.off_timeout;
    }

    pub fn set_off_timeout(&mut self, off_timeout: u8){
        self.off_timeout = off_timeout;
    }
}