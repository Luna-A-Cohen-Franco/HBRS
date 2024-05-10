#[derive(Debug)]
pub enum HbrsError {
    DataLengthExceedsLimit,
    BadMacAddressLength,
    BadMacAddressValues,
    BadIpv4AddressLength,
    InvalidScanDataLength,
    BadSSID,
}

/*
    No need to save error messeages as individual constants as their value will only
    be declared once inside this impl block and nowhere else.
*/
impl HbrsError {
    pub fn get_messeage(&self) -> String {
        match self {
            HbrsError::DataLengthExceedsLimit => "Data length exceeds string size".to_string(),
            HbrsError::BadMacAddressLength => "Invalid MAC address size".to_string(),
            HbrsError::BadMacAddressValues => "Invalid MAC address values".to_string(),
            HbrsError::BadIpv4AddressLength => "Invalid IPv4 address size".to_string(),
            HbrsError::InvalidScanDataLength => "Invalid scan data length".to_string(),
            HbrsError::BadSSID => "Bad SSID".to_string(),
        }
    }
}