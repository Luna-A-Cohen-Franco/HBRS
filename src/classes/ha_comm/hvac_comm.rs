#[derive(Debug, Clone)]
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

    pub fn get_comm_ref(&self) -> &u8{
        &self.comm
    }

    pub fn get_comm_mut(&mut self) -> &mut u8{
        &mut self.comm
    }
}
