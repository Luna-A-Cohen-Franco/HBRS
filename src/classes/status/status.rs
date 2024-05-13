use crate::classes::endpoint::endpoint_value::EndpointValue;

use super::{status_hvac::StatusHVAC, status_light::StatusLight};

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
}
/*
public class HACommand_Status
{
	public EndpointValues pTheEndpointValues;

	protected byte StatusType { get; private set; }

	private HACommand_StatusLight Light { get; set; }

	private HACommand_StatusHVAC HVAC { get; set; }

	//private HACommand_StatusCurtain Curtain { get; set; }

	//private HACommand_StatusIAS IASSensor { get; set; }

	//private HACommand_StatusDoorLock DoorLock { get; set; }

	public EndpointValues TheEndpointValues
	{
		get
		{
			switch (StatusType)
			{
			case 4:
				//pTheEndpointValues.Values = Curtain.EndpointValues;
				break;
			case 6:
				pTheEndpointValues.Values = HVAC.EndpointValues;
				break;
			case 1:
				pTheEndpointValues.Values = Light.EndpointValues;
				break;
			case 8:
				//pTheEndpointValues.Values = IASSensor.EndpointValues;
				break;
			case 7:
				//pTheEndpointValues.Values = DoorLock.EndpointValues;
				break;
			}
			return pTheEndpointValues;
		}
	}

	public HACommand_Status()
	{
		pTheEndpointValues = new EndpointValues();
	}

	public void SetBytes(byte[] data, int headerOffset)
	{
		StatusType = data[headerOffset];
		switch (StatusType)
		{
		case 4:
			/*if (Curtain == null)
			{
				Curtain = new HACommand_StatusCurtain();
			}
			Curtain.SetBytes(data, headerOffset + 1);*/
			break;
		case 6:
			if (HVAC == null)
			{
				HVAC = new HACommand_StatusHVAC();
			}
			HVAC.SetBytes(data, headerOffset + 1);
			break;
		case 1:
			if (Light == null)
			{
				Light = new HACommand_StatusLight();
			}
			Light.SetBytes(data, headerOffset + 1);
			break;
		case 8:
			/*if (IASSensor == null)
			{
				IASSensor = new HACommand_StatusIAS();
			}
			IASSensor.SetBytes(data, headerOffset + 1);*/
			break;
		case 7:
			/*if (DoorLock == null)
			{
				DoorLock = new HACommand_StatusDoorLock();
			}
			DoorLock.SetBytes(data, headerOffset + 1);*/
			break;
		case 2:
		case 3:
		case 5:
			break;
		}
	}
}

*/