pub enum OtherConsts{
    KeyLength
}

impl OtherConsts{
    pub fn get_value(&self) -> usize{
        match self{
            OtherConsts::KeyLength => 33,
        }
    }
}