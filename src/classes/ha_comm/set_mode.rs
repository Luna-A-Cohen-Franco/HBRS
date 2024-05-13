#[derive(Debug)]
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

    pub fn set_mode(&mut self, mode: u8){
        self.mode = mode;
    }

    pub fn set_fan_mode(&mut self, fan_mode: u8){
        self.fan_mode = fan_mode;
    }

    pub fn set_flags(&mut self, flags: u8){
        self.flags = flags;
    }

    pub fn set_desired_temp_b0(&mut self, desired_temp_b0: u8){
        self.desired_temp_b0 = desired_temp_b0;
    }

    pub fn set_desired_temp_b1(&mut self, desired_temp_b1: u8){
        self.desired_temp_b1 = desired_temp_b1;
    }

    pub fn get_mode(&self) -> u8{
        return self.mode;
    }

    pub fn get_fan_mode(&self) -> u8{
        return self.fan_mode;
    }

    pub fn get_flags(&self) -> u8{
        return self.flags;
    }

    pub fn get_desired_temp_b0(&self) -> u8{
        return self.desired_temp_b0;
    }

    pub fn get_desired_temp_b1(&self) -> u8{
        return self.desired_temp_b1;
    }

    
}