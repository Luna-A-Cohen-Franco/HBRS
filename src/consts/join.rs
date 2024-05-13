pub enum HACommJoinConsts{
    Enumerate,
    EnumerateResponse,
    Join,
    EnumerateAll,
    Scan,
    Scan_Response,
    GetCloudNotifIP,
    Set_Cloud_NotifIP,
    Key_Request,
    Set_Key,
    Subscribe,
    Unsubscribe,
}

impl HACommJoinConsts{
    pub fn get_value(&self) -> u8{
        match self{
            HACommJoinConsts::Enumerate => 1,
            HACommJoinConsts::EnumerateResponse => 2,
            HACommJoinConsts::Join => 3,
            HACommJoinConsts::EnumerateAll => 4,
            HACommJoinConsts::Scan => 5,
            HACommJoinConsts::ScanResponse => 6,
            HACommJoinConsts::GetCloudNotifIP => 7,
            HACommJoinConsts::SetCloudNotifIP => 8,
            HACommJoinConsts::Key_Request => 9,
            HACommJoinConsts::Set_Key => 10,
            HACommJoinConsts::Subscribe => 12,
            HACommJoinConsts::Unsubscribe => 13,
        }
    }
}