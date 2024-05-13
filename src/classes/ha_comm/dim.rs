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
}