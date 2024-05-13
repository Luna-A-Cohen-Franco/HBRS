pub struct HVACComm{
    comm: u8,
}

impl HVACComm{
    pub fn new() -> Self{
        Self{
            comm: 0,
        }
    }

    pub fn get_byte(&self) -> u8{
        return self.comm;
    }
}
