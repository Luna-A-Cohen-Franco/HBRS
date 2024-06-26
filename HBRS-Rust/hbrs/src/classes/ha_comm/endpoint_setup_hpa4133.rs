use crate::utils::byte_arrays_helper::ByteArraysHelper;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct EndpointSetupHPA4133{
    start_type: u8,
}

impl EndpointSetupHPA4133{
    pub fn new() -> Self{
        Self{
            start_type: 1,
        }
    }

    pub fn get_bytes(&self) -> Vec<u8>{
        let second = vec![0; 8];
        return ByteArraysHelper::combine_1v_1b(&second, self.start_type);
    }

    pub fn get_start_type_ref(&self) -> &u8{
        return &self.start_type;
    }

    pub fn get_start_type_mut(&mut self) -> &mut u8{
        return &mut self.start_type;
    }
}
