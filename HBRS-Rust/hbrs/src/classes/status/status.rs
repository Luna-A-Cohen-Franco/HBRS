use crate::classes::endpoint::endpoint_value::EndpointValue;
use serde::{Serialize, Deserialize};

use super::{status_hvac::StatusHVAC, status_light::StatusLight};

#[derive(Serialize, Deserialize)]
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

	pub fn get_endpoint_values_ref(&self) -> &Vec<EndpointValue> {
		&self.endpoint_values
	}
	
	pub fn get_endpoint_values_mut(&mut self) -> &mut Vec<EndpointValue> {
		&mut self.endpoint_values
	}
	
	pub fn get_status_type_ref(&self) -> &u8 {
		&self.status_type
	}
	
	pub fn get_status_type_mut(&mut self) -> &mut u8 {
		&mut self.status_type
	}
	
	pub fn get_light(&self) -> Option<&StatusLight> {
        self.light.as_ref()
    }

    pub fn set_light(&mut self, light: Option<StatusLight>) {
        self.light = light;
    }

    pub fn get_hvac(&self) -> Option<&StatusHVAC> {
        self.hvac.as_ref()
    }

    pub fn set_hvac(&mut self, hvac: Option<StatusHVAC>) {
        self.hvac = hvac;
    }

	/*
	pub fn get_curtain_ref(&self) -> Option<&StatusCurtain> {
		self.curtain.as_ref()
	}

	pub fn get_curtain_mut(&mut self) -> Option<&mut StatusCurtain> {
		self.curtain.as_mut()
	}

	pub fn get_ias_sensor_ref(&self) -> Option<&StatusIAS> {
		self.ias_sensor.as_ref()
	}

	pub fn get_ias_sensor_mut(&mut self) -> Option<&mut StatusIAS> {
		self.ias_sensor.as_mut()
	}

	pub fn get_door_lock_ref(&self) -> Option<&StatusDoorLock> {
		self.door_lock.as_ref()
	}

	pub fn get_door_lock_mut(&mut self) -> Option<&mut StatusDoorLock> {
		self.door_lock.as_mut()
	}
	*/
}

