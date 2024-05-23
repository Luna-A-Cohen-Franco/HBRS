use crate::utils::byte_arrays_helper::ByteArraysHelper;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
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

    pub fn get_off_timeout_ref(&self) -> &u8{
        return &self.off_timeout;
    }

    pub fn get_off_timeout_mut(&mut self) -> &mut u8{
        return &mut self.off_timeout;
    }
}