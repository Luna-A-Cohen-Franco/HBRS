use crate::classes::{join::join::Join, status::status::Status};

use super::{custom_comm::CustomComm, dim::Dim, header::Header, hvac_comm::HVACComm, ping::Ping, set_auto_off::SetAutoOff, set_mode::SetMode};

pub struct HACommand{
    header: Header,
    join: Option<Join>,
    ping: Option<Ping>,
    status: Option<Status>,
    hvac_set_mode: Option<SetMode>,
    hvac_command: Option<HVACComm>,
    custom_command: Option<CustomComm>,
    dim_cmd: Option<Dim>,
    set_auto_off: Option<SetAutoOff>,
    // door_lock: Option<DoorLock>,
    // curtain_position: Option<CurtainPosition>,
    // ilearn_data_rsp: Option<ILearnDataRsp>,
    // activate_scene: Option<ActivateScene>,
    // HPA4911_status_rsp: Option<HPA4911Status>,
    // HPA4413_status_rsp: Option<HPA4413Status>,
    // HPA4414_status_rsp: Option<HPA4414Status>,
    // fade_rate: Option<FadeRate>,
}

impl HACommand{
    fn new() -> Self{
        Self{
            header: Header::new(),
            join: None,
            ping: None,
            status: None,
            hvac_set_mode: None,
            hvac_command: None,
            custom_command: None,
            dim_cmd: None,
            set_auto_off: None,
        }
    }

    pub fn clear_bytes(&mut self){
        self.header.clear_bytes();
        self.join = None;
        self.ping = None;
        self.custom_command = None;
        self.set_auto_off = None;
    }

    pub fn get_bytes(&self) -> Vec<u8>{
        match self.header.get_id(){
            1 | 2 | 34 | 40 | 41 | 164 | 228 => return self.header.get_bytes(),
            3 => return vec![self.dim_cmd.as_ref().unwrap().get_byte()],
            // 39 =>  return self.curtain_position.as_ref().unwrap().get_bytes(),
            97 => return self.hvac_set_mode.as_ref().unwrap().get_bytes(),
            98 => return vec![self.hvac_command.as_ref().unwrap().get_byte()],
            // 145 => return self.door_lock.as_ref().unwrap().get_bytes(),
            161 => return self.join.as_ref().unwrap().get_bytes(),
            162 => return self.custom_command.as_ref().unwrap().get_bytes(),
            175 => return self.header.get_bytes(),
            // 177 => return self.fade_rate.as_ref().unwrap().get_bytes(),
            178 => return self.set_auto_off.as_ref().unwrap().get_bytes(),
            224 => return self.ping.as_ref().unwrap().get_bytes(),
            // 232 => return self.activate_scene.as_ref().unwrap().get_bytes(),
            _ => return Vec::new(),
        }
    }
}