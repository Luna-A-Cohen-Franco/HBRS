pub enum ResponseItem{
    SSIDLength,
    SecurityTypePos,
    EncryptionTypePos,
    RSSIPos,
}

impl ResponseItem{
    pub fn get_value(&self) -> usize{
        match self{
            Self::SSIDLength => 32,
            Self::SecurityTypePos => 33,
            Self::EncryptionTypePos => 34,
            Self::RSSIPos => 35,
        }
    }
}