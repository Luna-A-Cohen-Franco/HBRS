use crate::consts::errors::HbrsError;

#[derive(Debug, Clone)]
pub struct EnumRes{
    app_ver: String,
    device_model: String,
    firmware_ver: String,
}

impl EnumRes{
    pub fn new(data: &mut Vec<u8>, header_offset: usize) -> Result<Self, HbrsError>{
        let num = data.len() - header_offset;
        let app_ver_bytes = vec![];
        let mut firmware_ver = String::new();
        let mut device_model = String::new();

        data[header_offset as usize..(header_offset + num) as usize].copy_from_slice(&app_ver_bytes);
        
        let app_ver = match String::from_utf8(app_ver_bytes.clone()){
            Ok(s) => s,
            Err(_) => return Err(HbrsError::BadJoinEnumRes),
        };

        let app_ver_split = app_ver.split(",").collect::<Vec<&str>>();

        if app_ver_split.len() != 0{
            device_model = app_ver_split[0].to_string();
        }

        if app_ver_split.len() > 1{
            firmware_ver = app_ver_split[1].to_string();
        }

        Ok(Self{
            app_ver,
            device_model,
            firmware_ver,
        })
    }

    pub fn set_bytes(&mut self, mut data: Vec<u8>, header_offset: usize) -> Result<bool, HbrsError>{
        let num = data.len() - header_offset;
        let app_ver_bytes = vec![];

        data[header_offset as usize..(header_offset + num) as usize].copy_from_slice(&app_ver_bytes);
        
        let app_ver = match String::from_utf8(app_ver_bytes.clone()){
            Ok(s) => s,
            Err(_) => return Err(HbrsError::BadJoinEnumRes),
        };

        let app_ver_split = app_ver.split(",").collect::<Vec<&str>>();

        if app_ver_split.len() != 0{
            self.device_model = app_ver_split[0].to_string();
        }

        if app_ver_split.len() > 1{
            self.firmware_ver = app_ver_split[1].to_string();
        }

        return Ok(true)
    }

    pub fn get_bytes(&self) -> Vec<u8>{
        let mut bytes = vec![];
        bytes.extend_from_slice(self.app_ver.as_bytes());
        return bytes;
    }

    pub fn get_app_ver(&self) -> String{
        return self.app_ver.clone();
    }

    pub fn get_device_model(&self) -> String{
        return self.device_model.clone();
    }

    pub fn get_firmware_ver(&self) -> String{
        return self.firmware_ver.clone();
    }

    pub fn set_app_ver(&mut self, app_ver: String){
        self.app_ver = app_ver;
    }

    pub fn set_device_model(&mut self, device_model: String){
        self.device_model = device_model;
    }

    pub fn set_firmware_ver(&mut self, firmware_ver: String){
        self.firmware_ver = firmware_ver;
    }
}
