pub struct HACommHVACComm{
    comm: u8,
}

impl HACommHVACComm{
    pub fn new() -> Self{
        Self{
            comm: 0,
        }
    }

    pub fn get_byte(&self) -> u8{
        return self.comm;
    }
}
