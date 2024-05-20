use crate::classes::{join::join::Join, status::status::Status};
use serde::{Serialize, Deserialize};
use crate::consts::other::Other;

use super::{custom_comm::CustomComm, dim::Dim, header::Header, hvac_comm::HVACComm, ping::Ping, set_auto_off::SetAutoOff, set_mode::SetMode};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
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
    pub fn new() -> Self{
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
        match self.header.get_id_ref(){
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

    pub fn set_bytes(&mut self, data: &mut Vec<u8>, subcomm_wait_res: u8){
        self.header.set_bytes(&data).unwrap();

        match *self.header.get_id_ref(){
            161 => {
                if self.join.is_none(){
                    self.join = Some(Join::new());
                }

                self.join.as_mut().unwrap().set_bytes(data, Other::HeaderOffset.get_value(), subcomm_wait_res).unwrap();
            },
            253 => {
                if self.status.is_none(){
                    self.status = Some(Status::new());
                }

                self.status.as_mut().unwrap().set_bytes(&data, Other::HeaderOffset.get_value());
            },
            162 => {
                if self.custom_command.is_none(){
                    self.custom_command = Some(CustomComm::new());
                }

                self.custom_command.as_mut().unwrap().set_bytes(&data, Other::HeaderOffset.get_value());
            },
            _ => {}
        }
    }

    pub fn get_header_ref(&self) -> &Header {
        &self.header
    }
    
    pub fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
    
    pub fn get_join(&self) -> Option<&Join> {
        self.join.as_ref()
    }

    pub fn set_join(&mut self, join: Option<Join>) {
        self.join = join;
    }

    pub fn get_ping(&self) -> Option<&Ping> {
        self.ping.as_ref()
    }

    pub fn set_ping(&mut self, ping: Option<Ping>) {
        self.ping = ping;
    }

    pub fn get_status(&self) -> Option<&Status> {
        self.status.as_ref()
    }

    pub fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }

    pub fn get_hvac_set_mode(&self) -> Option<&SetMode> {
        self.hvac_set_mode.as_ref()
    }

    pub fn set_hvac_set_mode(&mut self, hvac_set_mode: Option<SetMode>) {
        self.hvac_set_mode = hvac_set_mode;
    }

    pub fn get_hvac_command(&self) -> Option<&HVACComm> {
        self.hvac_command.as_ref()
    }

    pub fn set_hvac_command(&mut self, hvac_command: Option<HVACComm>) {
        self.hvac_command = hvac_command;
    }

    pub fn get_custom_command(&self) -> Option<&CustomComm> {
        self.custom_command.as_ref()
    }

    pub fn set_custom_command(&mut self, custom_command: Option<CustomComm>) {
        self.custom_command = custom_command;
    }

    pub fn get_dim_cmd(&self) -> Option<&Dim> {
        self.dim_cmd.as_ref()
    }

    pub fn set_dim_cmd(&mut self, dim_cmd: Option<Dim>) {
        self.dim_cmd = dim_cmd;
    }

    pub fn get_set_auto_off(&self) -> Option<&SetAutoOff> {
        self.set_auto_off.as_ref()
    }

    pub fn set_set_auto_off(&mut self, set_auto_off: Option<SetAutoOff>) {
        self.set_auto_off = set_auto_off;
    }

    /*
    pub fn get_door_lock_ref(&self) -> Option<&DoorLock> {
        self.door_lock.as_ref()
    }
    
    pub fn get_door_lock_mut(&mut self) -> Option<&mut DoorLock> {
        self.door_lock.as_mut()
    }
    
    pub fn get_curtain_position_ref(&self) -> Option<&CurtainPosition> {
        self.curtain_position.as_ref()
    }
    
    pub fn get_curtain_position_mut(&mut self) -> Option<&mut CurtainPosition> {
        self.curtain_position.as_mut()
    }
    
    pub fn get_ilearn_data_rsp_ref(&self) -> Option<&ILearnDataRsp> {
        self.ilearn_data_rsp.as_ref()
    }
    
    pub fn get_ilearn_data_rsp_mut(&mut self) -> Option<&mut ILearnDataRsp> {
        self.ilearn_data_rsp.as_mut()
    }
    
    pub fn get_activate_scene_ref(&self) -> Option<&ActivateScene> {
        self.activate_scene.as_ref()
    }
    
    pub fn get_activate_scene_mut(&mut self) -> Option<&mut ActivateScene> {
        self.activate_scene.as_mut()
    }
    
    pub fn get_HPA4911_status_rsp_ref(&self) -> Option<&HPA4911Status> {
        self.HPA4911_status_rsp.as_ref()
    }
    
    pub fn get_HPA4911_status_rsp_mut(&mut self) -> Option<&mut HPA4911Status> {
        self.HPA4911_status_rsp.as_mut()
    }
    
    pub fn get_HPA4413_status_rsp_ref(&self) -> Option<&HPA4413Status> {
        self.HPA4413_status_rsp.as_ref()
    }
    
    pub fn get_HPA4413_status_rsp_mut(&mut self) -> Option<&mut HPA4413Status> {
        self.HPA4413_status_rsp.as_mut()
    }
    
    pub fn get_HPA4414_status_rsp_ref(&self) -> Option<&HPA4414Status> {
        self.HPA4414_status_rsp.as_ref()
    }
    
    pub fn get_HPA4414_status_rsp_mut(&mut self) -> Option<&mut HPA4414Status> {
        self.HPA4414_status_rsp.as_mut()
    }
    
    pub fn get_fade_rate_ref(&self) -> Option<&FadeRate> {
        self.fade_rate.as_ref()
    }
    
    pub fn get_fade_rate_mut(&mut self) -> Option<&mut FadeRate> {
        self.fade_rate.as_mut()
    }
    */    
}