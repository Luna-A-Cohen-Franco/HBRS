use super::{status_hvac::StatusHVAC, status_light::StatusLight};

pub struct Status{
	status_type: u8,
	light: Option<StatusLight>,
	hvac: Option<StatusHVAC>,
	//curtain: Option<StatusCurtain>,
	//ias_sensor: Option<StatusIAS>,
	//door_lock: Option<StatusDoorLock>,
}

impl Status{
	
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