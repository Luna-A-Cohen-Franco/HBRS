use crate::classes::{join::join::Join, status::status::Status};

use super::{custom_comm::CustomComm, dim::Dim, header::Header, hvac_comm::HVACComm, ping::Ping, set_auto_off::SetAutoOff, set_mode::SetMode};

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

    pub fn get_header_ref(&self) -> &Header {
        &self.header
    }
    
    pub fn get_header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
    
    pub fn get_join_ref(&self) -> Option<&Join> {
        self.join.as_ref()
    }
    
    pub fn get_join_mut(&mut self) -> Option<&mut Join> {
        self.join.as_mut()
    }
    
    pub fn get_ping_ref(&self) -> Option<&Ping> {
        self.ping.as_ref()
    }
    
    pub fn get_ping_mut(&mut self) -> Option<&mut Ping> {
        self.ping.as_mut()
    }
    
    pub fn get_status_ref(&self) -> Option<&Status> {
        self.status.as_ref()
    }
    
    pub fn get_status_mut(&mut self) -> Option<&mut Status> {
        self.status.as_mut()
    }
    
    pub fn get_hvac_set_mode_ref(&self) -> Option<&SetMode> {
        self.hvac_set_mode.as_ref()
    }
    
    pub fn get_hvac_set_mode_mut(&mut self) -> Option<&mut SetMode> {
        self.hvac_set_mode.as_mut()
    }
    
    pub fn get_hvac_command_ref(&self) -> Option<&HVACComm> {
        self.hvac_command.as_ref()
    }
    
    pub fn get_hvac_command_mut(&mut self) -> Option<&mut HVACComm> {
        self.hvac_command.as_mut()
    }
    
    pub fn get_custom_command_ref(&self) -> Option<&CustomComm> {
        self.custom_command.as_ref()
    }
    
    pub fn get_custom_command_mut(&mut self) -> Option<&mut CustomComm> {
        self.custom_command.as_mut()
    }
    
    pub fn get_dim_cmd_ref(&self) -> Option<&Dim> {
        self.dim_cmd.as_ref()
    }
    
    pub fn get_dim_cmd_mut(&mut self) -> Option<&mut Dim> {
        self.dim_cmd.as_mut()
    }
    
    pub fn get_set_auto_off_ref(&self) -> Option<&SetAutoOff> {
        self.set_auto_off.as_ref()
    }
    
    pub fn get_set_auto_off_mut(&mut self) -> Option<&mut SetAutoOff> {
        self.set_auto_off.as_mut()
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