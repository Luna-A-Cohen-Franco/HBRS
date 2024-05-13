use crate::classes::endpoint::endpoint_value::EndpointValue;

use super::{status_hvac::StatusHVAC, status_light::StatusLight};

#[derive(Debug, Clone)]
pub struct Status{
	endpoint_values: Vec<EndpointValue>,
	status_type: u8,
	light: Option<StatusLight>,
	hvac: Option<StatusHVAC>,
	//curtain: Option<StatusCurtain>,
	//ias_sensor: Option<StatusIAS>,
	//door_lock: Option<StatusDoorLock>,
}

impl Status{
	pub fn new() -> Self{
		Self{
			endpoint_values: Vec::new(),
			status_type: 0,
			light: None,
			hvac: None,
			//curtain: None,
			//ias_sensor: None,
			//door_lock: None,
		}
	}

	pub fn update_status(&mut self, status_type: u8){
		self.status_type = status_type;
		self.endpoint_values = self.match_endpoint_values();
	}

	pub fn match_endpoint_values(&mut self) -> Vec<EndpointValue>{
		match self.status_type{
			1 => {
				if self.light.is_none(){
					self.light = Some(StatusLight::new());
				}
				
				return self.light.as_ref().unwrap().get_endpoint_values();
			},
			/*4 => {
				if self.curtain.is_none(){
					self.curtain = Some(StatusCurtain::new());
				}
				return self.curtain.as_ref().unwrap().getTheEndpointValues();
			},*/
			6 => {
				if self.hvac.is_none(){
					self.hvac = Some(StatusHVAC::new());
				}
				return self.hvac.as_ref().unwrap().get_endpoint_values();
			},
			/*7 => {
				if self.door_lock.is_none(){
					self.door_lock = Some(StatusDoorLock::new());
				}
				return self.door_lock.as_ref().unwrap().get_endpoint_values();
			},*/
			/*8 => {
				if self.ias_sensor.is_none(){
					self.ias_sensor = Some(StatusIAS::new());
				}
				return self.ias_sensor.as_ref().unwrap().get_endpoint_values();
			},*/
			_ => return Vec::new(),
		}
	}

	pub fn set_bytes(&mut self, data: &Vec<u8>, header_offset: usize) -> bool{
		self.status_type = data[header_offset];
		match self.status_type{
			/*4 => {
				if self.curtain.is_none(){
					self.curtain = Some(StatusCurtain::new());
				}
				self.curtain.as_mut().unwrap().set_bytes(data, header_offset + 1);
			},*/
			6 => {
				if self.hvac.is_none(){
					self.hvac = Some(StatusHVAC::new());
				}
				self.hvac.as_mut().unwrap().set_bytes(data, header_offset + 1);
			},
			1 => {
				if self.light.is_none(){
					self.light = Some(StatusLight::new());
				}
				self.light.as_mut().unwrap().set_bytes(data, header_offset + 1);
			},
			/*8 => {
				if self.ias_sensor.is_none(){
					self.ias_sensor = Some(StatusIAS::new());
				}
				self.ias_sensor.as_mut().unwrap().set_bytes(data, header_offset + 1);
			},*/
			/*7 => {
				if self.door_lock.is_none(){
					self.door_lock = Some(StatusDoorLock::new());
				}
				self.door_lock.as_mut().unwrap().set_bytes(data, header_offset + 1);
			},
			2 | 3 | 5 => {},*/
			_ => {},
		}
		
		return true;
	}

	pub fn get_endpoint_values(&self) -> Vec<EndpointValue>{
		return self.endpoint_values.iter().cloned().collect();
	}

	pub fn get_status_type(&self) -> u8{
		return self.status_type;
	}

	pub fn set_status_type(&mut self, status_type: u8){
		self.status_type = status_type;
	}

	pub fn get_light(&self) -> Option<StatusLight>{
		return self.light.clone();
	}

	pub fn set_light(&mut self, light: StatusLight){
		self.light = Some(light);
	}

	pub fn get_hvac(&self) -> Option<StatusHVAC>{
		return self.hvac.clone();
	}

	pub fn set_hvac(&mut self, hvac: StatusHVAC){
		self.hvac = Some(hvac);
	}

	/*pub fn get_curtain(&self) -> Option<StatusCurtain>{
		return self.curtain.clone();
	}

	pub fn set_curtain(&mut self, curtain: StatusCurtain){
		self.curtain = Some(curtain);
	}

	pub fn get_ias_sensor(&self) -> Option<StatusIAS>{
		return self.ias_sensor.clone();
	}

	pub fn set_ias_sensor(&mut self, ias_sensor: StatusIAS){
		self.ias_sensor = Some(ias_sensor);
	}

	pub fn get_door_lock(&self) -> Option<StatusDoorLock>{
		return self.door_lock.clone();
	}

	pub fn set_door_lock(&mut self, door_lock: StatusDoorLock){
		self.door_lock = Some(door_lock);
	}*/
}

