use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct SetMode{
    mode: u8,
    fan_mode: u8,
    flags: u8,
    desired_temp_b0: u8,
    desired_temp_b1: u8,
}

impl SetMode{
    pub fn new() -> Self{
        Self{
            mode: 0,
            fan_mode: 0,
            flags: 0,
            desired_temp_b0: 0,
            desired_temp_b1: 0,
        }
    }

    pub fn get_bytes(&self) -> Vec<u8>{
        return vec![self.mode, self.fan_mode, self.flags, self.desired_temp_b0, self.desired_temp_b1];
    }

    pub fn get_mode_ref(&self) -> &u8 {
        &self.mode
    }
    
    pub fn get_mode_mut(&mut self) -> &mut u8 {
        &mut self.mode
    }
    
    pub fn get_fan_mode_ref(&self) -> &u8 {
        &self.fan_mode
    }
    
    pub fn get_fan_mode_mut(&mut self) -> &mut u8 {
        &mut self.fan_mode
    }
    
    pub fn get_flags_ref(&self) -> &u8 {
        &self.flags
    }
    
    pub fn get_flags_mut(&mut self) -> &mut u8 {
        &mut self.flags
    }
    
    pub fn get_desired_temp_b0_ref(&self) -> &u8 {
        &self.desired_temp_b0
    }
    
    pub fn get_desired_temp_b0_mut(&mut self) -> &mut u8 {
        &mut self.desired_temp_b0
    }
    
    pub fn get_desired_temp_b1_ref(&self) -> &u8 {
        &self.desired_temp_b1
    }
    
    pub fn get_desired_temp_b1_mut(&mut self) -> &mut u8 {
        &mut self.desired_temp_b1
    }
}