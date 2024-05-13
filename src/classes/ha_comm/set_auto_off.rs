use crate::utils::byte_arrays_helper::ByteArraysHelper;

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
}