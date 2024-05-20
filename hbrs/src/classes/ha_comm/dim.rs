use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Dim{
    dim_value: u8,
}

impl Dim{
    pub fn new() -> Self{
        Self{
            dim_value: 0,
        }
    }

    pub fn get_byte(&self) -> u8{
        return self.dim_value;
    }

    pub fn get_dim_value_ref(&self) -> &u8{
        return &self.dim_value
    }

    pub fn get_dim_value_mut(&mut self) -> &mut u8{
        return &mut self.dim_value
    }
}