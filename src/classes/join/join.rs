use crate::{consts::errors::HbrsError, utils::byte_arrays_helper::ByteArraysHelper};

use super::{cloud_notif_ip::CloudNotifIP, enum_res::EnumRes, join_req::JoinReq, scan_res::ScanRes};

#[derive(Debug, Clone)]
pub struct Join{
    sub_command: u8,
    join_req: Option<JoinReq>,
    enum_res: Option<EnumRes>,
    scan_res: Option<ScanRes>,
    cloud_notif_ip: Option<CloudNotifIP>,
}

impl Join{
    pub fn new() -> Self{
        Self{
            sub_command: 0,
            join_req: None,
            enum_res: None,
            scan_res: None,
            cloud_notif_ip: None,
        }
    }

    pub fn get_bytes(&self) -> Vec<u8>{
        let data: Vec<u8> = vec![self.sub_command];

        if self.sub_command == 8 && self.enum_res.is_some(){
            return ByteArraysHelper::combine_2v(&data, &self.cloud_notif_ip.as_ref().unwrap().get_bytes());
        }

        if self.sub_command == 3 && self.join_req.is_some(){
            return ByteArraysHelper::combine_2v(&data, &self.join_req.as_ref().unwrap().get_bytes());
        }

        return data;
    }

    pub fn set_bytes(&mut self, mut data: Vec<u8>, header_offset: usize, sub_comm_res: u8) -> Result<bool, HbrsError>{
        if sub_comm_res == 2{
            if self.enum_res.is_none(){
                self.enum_res = match EnumRes::new(&mut data, header_offset + 1){
                    Ok(e) => Some(e),
                    Err(err) => return Err(err),
                };
                
                return Ok(true)
            }

            match self.enum_res.as_mut().unwrap().set_bytes(data, header_offset + 1){
                Ok(_) => return Ok(true),
                Err(err) => return Err(err),
            };
        }

        if sub_comm_res == 6 {
            if self.scan_res.is_none(){
                self.scan_res = Some(ScanRes::new());
            }

            match self.scan_res.as_mut().unwrap().add_new_item(data, header_offset + 1){
                Ok(_) => return Ok(true),
                Err(err) => return Err(err),
            };
        }

        return Ok(false);
    }

    pub fn get_sub_command(&self) -> u8{
        return self.sub_command;
    }

    pub fn set_sub_command(&mut self, sub_command: u8){
        self.sub_command = sub_command;
    }

    pub fn get_join_req(&self) -> Option<JoinReq>{
        return self.join_req.clone();
    }

    pub fn set_join_req(&mut self, join_req: JoinReq){
        self.join_req = Some(join_req);
    }

    pub fn get_enum_res(&self) -> Option<EnumRes>{
        return self.enum_res.clone();
    }

    pub fn set_enum_res(&mut self, enum_res: EnumRes){
        self.enum_res = Some(enum_res);
    }

    pub fn get_scan_res(&self) -> Option<ScanRes>{
        return self.scan_res.clone();
    }

    pub fn set_scan_res(&mut self, scan_res: ScanRes){
        self.scan_res = Some(scan_res);
    }

    pub fn get_cloud_notif_ip(&self) -> Option<CloudNotifIP>{
        return self.cloud_notif_ip.clone();
    }

    pub fn set_cloud_notif_ip(&mut self, cloud_notif_ip: CloudNotifIP){
        self.cloud_notif_ip = Some(cloud_notif_ip);
    }
}