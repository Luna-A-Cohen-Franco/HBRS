use crate::utils::byte_arrays_helper::ByteArraysHelper;

#[derive(Debug, Clone)]
pub struct EndpointSetupHPA4133{
    pub start_type: u8,
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

    pub fn get_start_type(&self) -> u8{
        return self.start_type;
    }

    pub fn set_start_type(&mut self, start_type: u8){
        self.start_type = start_type;
    }
}
