pub enum Other{
    KeyLength,
    HeaderOffset,
}

impl Other{
    pub fn get_value(&self) -> usize{
        match self{
            Self::KeyLength => 33,
            Self::HeaderOffset => 17,
        }
    }
}