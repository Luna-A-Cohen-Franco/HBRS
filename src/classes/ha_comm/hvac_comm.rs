#[derive(Debug)]
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

    pub fn set_byte(&mut self, comm: u8){
        self.comm = comm;
    }
}
