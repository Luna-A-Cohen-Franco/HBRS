pub struct SetMode{
    pub mode: u8,
    pub fan_mode: u8,
    pub flags: u8,
    pub desired_temp_b0: u8,
    pub desired_temp_b1: u8,
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
}