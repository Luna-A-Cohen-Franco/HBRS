using System;
using System.CodeDom.Compiler;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.IO;
using System.Net;
using System.Net.Sockets;
using System.Net.NetworkInformation;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Runtime.Versioning;
using System.Threading;
using System.Threading.Tasks;	
using System.Globalization;
using System.Text;

// TODO: HACommand_EndpointSetup_HPA4133
public class HACommand_EndpointSetup_HPA4133
{
	public byte StartType { get; set; }

	public HACommand_EndpointSetup_HPA4133()
	{
		StartType = 1;
	}

	public byte[] GetBytes()
	{
		byte[] second = new byte[8];
		return ByteArraysHelper.Combine(StartType, second);
	}
}

// TODO: UDPResponseCodes
public enum UDPResponseCodes
{
	None,
	Join_EnumerateResponse,
	Join_ScanResponseStarted,
	Join_SwitchCloudResponse,
	Join_JoinResponse
}

// TODO: UpdateStatusInfo
public class UpdateStatusInfo
{
	public bool IsACK { get; set; }

	public long EndpointID { get; set; }

	public string LocalIPAddress { get; set; }

	public List<EndpointValue> EndpointValues { get; set; }
}

// TODO: DeviceIPInformation
public class DeviceIPInformation
{
   public long DeviceID { get; set; }

   public string DeviceMac { get; set; }

   public string LocalIPAddress { get; set; }
}

// TODO: HACommand
public class HACommand
{
	public HACommandHeader Header;

	public HACommand_Join Join;

	public HACommand_Ping Ping;

	public HACommand_Status Status;

//		public HACommand_CurtainPosition CurtainPosition;

	public HACommand_SetMode HVACSetMode;

//		public HACommand_ActivateScene ActivateScene;

	public HACommand_HVACCommand HVACCommand;

	public HACommand_CustomCommand CustomCommand;

//		public HACommand_ILeanDataResponse ILearnDataRsp;

	//public HACommand_HPA4911Status HPA4911StatusRsp;

	//public HACommand_HPA4413Status HPA4413StatusRsp;

	//public HACommand_HPA4414Status HPA4414StatusRsp;

	public HACommand_Dim DimCmd;

	public HACommand_SetAutoOff SetAutoOff;

	//public HACommand_SetFadeRate FadeRate;

	//public HACommand_Doorlock Doorlock;

	public HACommand()
	{
		Header = new HACommandHeader();
	}

	public void ClearBytes()
	{
		Header.ProtocolVersion = 0;
		Header.SourceMAC = new byte[6];
		Header.DestinationMAC = new byte[6];
		Header.SequenceNumber = 0;
		Header.SourceEndpoint = 0;
		Header.DestinationEndpoint = 0;
		Header.CommandID = 0;
		if (Join != null)
		{
			Join = null;
		}
		if (Ping != null)
		{
			Ping = null;
		}
		if (CustomCommand != null)
		{
			CustomCommand = null;
		}
		if (SetAutoOff != null)
		{
			SetAutoOff = null;
		}
	}

	public byte[] GetBytes()
	{
		switch (Header.CommandID)
		{
		case 161:
			return ByteArraysHelper.Combine(Header.GetBytes(), Join.GetBytes());
		case 224:
			return ByteArraysHelper.Combine(Header.GetBytes(), Ping.GetBytes());
		case 228:
			return Header.GetBytes();
		case 34:
		case 40:
		case 41:
			return Header.GetBytes();
		case 39:
			return null;//ByteArraysHelper.Combine(Header.GetBytes(), CurtainPosition.Position);
		case 175:
			return Header.GetBytes();
		case 97:
			return ByteArraysHelper.Combine(Header.GetBytes(), HVACSetMode.GetBytes());
		case 232:
			return null;//ByteArraysHelper.Combine(Header.GetBytes(), ActivateScene.GetBytes());
		case 162:
			return ByteArraysHelper.Combine(Header.GetBytes(), CustomCommand.GetBytes());
		case 98:
			return ByteArraysHelper.Combine(Header.GetBytes(), HVACCommand.GetBytes());
		case 164:
			return Header.GetBytes();
		case 1:
			return Header.GetBytes();
		case 2:
			return Header.GetBytes();
		case 3:
			return ByteArraysHelper.Combine(Header.GetBytes(), DimCmd.GetBytes());
		case 178:
			return ByteArraysHelper.Combine(Header.GetBytes(), SetAutoOff.GetBytes());
		case 177:
			return null;// ByteArraysHelper.Combine(Header.GetBytes(), FadeRate.GetBytes());
		case 145:
			return null;//ByteArraysHelper.Combine(Header.GetBytes(), Doorlock.GetBytes());
		default:
			return null;
		}
	}

	public void SetBytes(byte[] data, byte subCommandToWaitAsResponse = 0)
	{
		if (data == null || data.Length == 0)
		{
			return;
		}
		try
		{
			Header.SetBytes(data);
			switch (Header.CommandID)
			{
			case 161:
				if (Join == null)
				{
					Join = new HACommand_Join();
				}
				Join.SetBytes(data, Header.HeaderOffset, subCommandToWaitAsResponse);
				break;
			case 253:
				if (Status == null)
				{
					Status = new HACommand_Status();
				}
				Status.SetBytes(data, Header.HeaderOffset);
				break;
			case 162:
				if (CustomCommand == null)
				{
					CustomCommand = new HACommand_CustomCommand();
				}
				CustomCommand.SetBytes(data, Header.HeaderOffset);
				if (CustomCommand.Command == 95)
				{
					/*if (ILearnDataRsp == null)
					{
						ILearnDataRsp = new HACommand_ILeanDataResponse();
					}
					ILearnDataRsp.SetBytes(data, Header.HeaderOffset + 1);*/
				}
				else if (CustomCommand.Command == 92)
				{
					/*if (HPA4911StatusRsp == null)
					{
						HPA4911StatusRsp = new HACommand_HPA4911Status();
					}
					HPA4911StatusRsp.SetBytes(data, Header.HeaderOffset + 1);*/
				}
				else if (CustomCommand.Command == 92)
				{
					/*if (HPA4413StatusRsp == null)
					{
						HPA4413StatusRsp = new HACommand_HPA4413Status();
					}
					HPA4413StatusRsp.SetBytes(data, Header.HeaderOffset + 1);*/
				}
				else if (CustomCommand.Command == 92)
				{
					/*if (HPA4414StatusRsp == null)
					{
						HPA4414StatusRsp = new HACommand_HPA4414Status();
					}
					HPA4414StatusRsp.SetBytes(data, Header.HeaderOffset + 1);*/
				}
				break;
			}
		}
		catch (Exception)
		{
		}
	}
}

// TODO: HACommandHeader
public class HACommandHeader
{
	public byte ProtocolVersion;

	public byte[] SourceMAC;

	public byte[] DestinationMAC;

	public byte SequenceNumber;

	public byte SourceEndpoint;

	public byte DestinationEndpoint;

	public byte CommandID;

	public string MACOfLastResponseAsStr => ByteArraysHelper.MacAddressBytesToString(MACOfLastResponse);

	public byte[] MACOfLastResponse { get; set; }

	public byte HeaderOffset => 17;

	public HACommandHeader()
	{
		ProtocolVersion = 0;
		SourceMAC = new byte[6];
		DestinationMAC = new byte[6];
		SequenceNumber = 0;
		SourceEndpoint = 0;
		DestinationEndpoint = 0;
		CommandID = 0;
	}

	public byte[] GetBytes()
	{
		byte[] array = new byte[HeaderOffset];
		int num = 0;
		array[num] = ProtocolVersion;
		num++;
		byte[] sourceMAC = SourceMAC;
		foreach (byte b in sourceMAC)
		{
			array[num] = b;
			num++;
		}
		num = 7;
		sourceMAC = DestinationMAC;
		foreach (byte b2 in sourceMAC)
		{
			array[num] = b2;
			num++;
		}
		num = 13;
		array[num] = SequenceNumber;
		num++;
		array[num] = SourceEndpoint;
		num++;
		array[num] = DestinationEndpoint;
		num++;
		array[num] = CommandID;
		return array;
	}

	public void SetBytes(byte[] data)
	{
		int num = 0;
		ProtocolVersion = data[num];
		num++;
		for (int i = 0; i < 6; i++)
		{
			SourceMAC[i] = data[num + i];
		}
		MACOfLastResponse = SourceMAC;
		num = 7;
		for (int j = 0; j < 6; j++)
		{
			DestinationMAC[j] = data[num + j];
		}
		num = 13;
		SequenceNumber = data[num];
		num++;
		SourceEndpoint = data[num];
		num++;
		DestinationEndpoint = data[num];
		num++;
		CommandID = data[num];
	}
}
// TODO: HACommand_Join_JoinRequest
public class HACommand_Join_JoinRequest
{
	private byte[] SSID;

	private byte SecurityType;

	private byte EncryptionType;

	private byte[] Key;

	public HACommand_Join_JoinRequest(byte[] sSID, byte securityType, byte encryptionType, byte[] key)
	{
		SSID = sSID;
		SecurityType = securityType;
		EncryptionType = encryptionType;
		Key = ByteArraysHelper.CopyArrayFromStringWithFill(key, 33);
	}

	public byte[] GetBytes()
	{
		byte[] first = ByteArraysHelper.Combine(SSID, SecurityType);
		byte[] second = ByteArraysHelper.Combine(EncryptionType, Key);
		return ByteArraysHelper.Combine(first, second);
	}
}
// TODO: HACommand_Join_ScanResponseItem
public class HACommand_Join_ScanResponseItem
{
	public string SSIDAsStr => Encoding.UTF8.GetString(SSID, 0, (SSID != null) ? SSID.Length : 0);

	public byte[] SSID { get; private set; }

	public byte SecurityType { get; private set; }

	public byte EncryptionType { get; private set; }

	public byte RSSI { get; internal set; }

	public HACommand_Join_ScanResponseItem(byte[] data, int headerOffset)
	{
		if (SSID == null)
		{
			SSID = new byte[33];
		}
		for (int i = 0; i < 33; i++)
		{
			SSID[i] = 0;
		}
		Buffer.BlockCopy(data, headerOffset, SSID, 0, 32);
		SecurityType = data[headerOffset + 33];
		EncryptionType = data[headerOffset + 34];
		RSSI = data[headerOffset + 35];
	}
}
// TODO:  HACommand_Join_ScanResponse
public class HACommand_Join_ScanResponse
{
	public List<HACommand_Join_ScanResponseItem> ListOfWifis { get; set; }

	public HACommand_Join_ScanResponse()
	{
		ListOfWifis = new List<HACommand_Join_ScanResponseItem>();
	}

	public void AddNewItem(byte[] data, int headerOffset)
	{
		if (data == null || data.Length != 54)
		{
			return;
		}
		HACommand_Join_ScanResponseItem NewItem = new HACommand_Join_ScanResponseItem(data, headerOffset);
		if (Array.TrueForAll(NewItem.SSID, (byte val) => val == 0) || string.IsNullOrWhiteSpace(NewItem.SSIDAsStr) || NewItem.RSSI <= 20)
		{
			return;
		}
		HACommand_Join_ScanResponseItem hACommand_Join_ScanResponseItem = ListOfWifis.Find((HACommand_Join_ScanResponseItem i) => i.SSIDAsStr.ToLower() == NewItem.SSIDAsStr.ToLower());
		if (hACommand_Join_ScanResponseItem != null)
		{
			if (NewItem.RSSI > hACommand_Join_ScanResponseItem.RSSI)
			{
				hACommand_Join_ScanResponseItem.RSSI = NewItem.RSSI;
			}
		}
		else
		{
			ListOfWifis.Add(NewItem);
		}
	}
}
// TODO: HACommand_Join_EnumResponse
public class HACommand_Join_EnumResponse
{
	private byte[] AppVersion { get; set; }

	public string DeviceModel { get; private set; }

	public string FirmwareVersion { get; private set; }

	public HACommand_Join_EnumResponse(byte[] data, int headerOffset)
	{
		int num = data.Length - headerOffset;
		if (AppVersion == null)
		{
			AppVersion = new byte[num];
		}
		Buffer.BlockCopy(data, headerOffset, AppVersion, 0, num);
		string[] array = Encoding.UTF8.GetString(AppVersion, 0, (AppVersion != null) ? AppVersion.Length : 0).Split(new char[1] { ',' });
		if (array != null)
		{
			if (array.Length != 0)
			{
				DeviceModel = array[0];
			}
			if (array.Length > 1)
			{
				FirmwareVersion = array[1];
			}
		}
	}
}
// TODO: HACommand_Join_CloudNotificationIP
public class HACommand_Join_CloudNotificationIP
{
	private byte[] Address;

	public byte[] GetBytes()
	{
		return Address;
	}

	public HACommand_Join_CloudNotificationIP(string cloudIPAddress)
	{
		Address = ByteArraysHelper.CopyArrayFromStringWithFill(cloudIPAddress, 16);
	}
}

// TODO: HACommand_Join
public class HACommand_Join
{
	public byte SubCommand;

	public HACommand_Join_JoinRequest JoinRequest;

	public HACommand_Join_EnumResponse EnumResponse;

	public HACommand_Join_ScanResponse ScanResponse;

	public HACommand_Join_CloudNotificationIP CloudNotificationIP;

	public byte[] GetBytes()
	{
		byte[] array = new byte[1] { SubCommand };
		if (SubCommand == 8)
		{
			return ByteArraysHelper.Combine(array, CloudNotificationIP.GetBytes());
		}
		if (SubCommand == 3)
		{
			return ByteArraysHelper.Combine(array, JoinRequest.GetBytes());
		}
		return array;
	}

	public void SetBytes(byte[] data, int headerOffset, byte subCommandToWaitAsResponse)
	{
		switch (subCommandToWaitAsResponse)
		{
		case 2:
			if (EnumResponse == null)
			{
				EnumResponse = new HACommand_Join_EnumResponse(data, headerOffset + 1);
			}
			break;
		case 6:
			if (ScanResponse == null)
			{
				ScanResponse = new HACommand_Join_ScanResponse();
			}
			ScanResponse.AddNewItem(data, headerOffset + 1);
			break;
		}
	}
}

// TODO: HACommand_Join_Constants
public static class HACommand_Join_Constants
{
	public const byte SubCommand_Enumerate = 1;

	public const byte SubCommand_Enumerate_Response = 2;

	public const byte SubCommand_Join = 3;

	public const byte SubCommand_Enumerate_All = 4;

	public const byte SubCommand_Scan = 5;

	public const byte SubCommand_Scan_Response = 6;

	public const byte SubCommand_Get_Cloud_Notification_IP = 7;

	public const byte SubCommand_Set_Cloud_Notification_IP = 8;

	public const byte SubCommand_Key_Request = 9;

	public const byte SubCommand_Set_Key = 10;

	public const byte SubCommand_Subscribe = 12;

	public const byte SubCommand_Unsubscribe = 13;
}
// TODO: HACommand_Ping
public class HACommand_Ping
{
	private byte Length { get; set; }

	private byte[] Data { get; set; }

	public HACommand_Ping()
	{
		Length = 1;
		Data = new byte[1] { 65 };
	}

	public byte[] GetBytes()
	{
		return ByteArraysHelper.Combine(Length, Data);
	}
}
// TODO: HACommand_Dim
public class HACommand_Dim
{
	public byte DimValue { get; set; }

	public byte GetBytes()
	{
		return DimValue;
	}
}
// TODO: HACommand_SetAutoOff
public class HACommand_SetAutoOff
{
	public byte OffTimeout { get; set; }

	public byte[] GetBytes()
	{
		ByteArraysHelper.WORDToPortable16((int)OffTimeout, out var b, out var b2);
		return ByteArraysHelper.Combine(b, b2);
	}
}
// TODO: HACommand_CustomCommand
public class HACommand_CustomCommand
{
	public byte Command { get; set; }

	public byte[] Data { get; set; }

	public void SetBytes(byte[] data, int headerOffset)
	{
		Command = data[headerOffset];
		Data = null;
	}

	public byte[] GetBytes()
	{
		if (Data == null || Data.Length == 0)
		{
			return new byte[1] { Command };
		}
		return ByteArraysHelper.Combine(Command, Data);
	}
}
// TODO:  HACommand_SetMode
public class HACommand_SetMode
{
	public byte Mode { get; set; }

	public byte FanMode { get; set; }

	public byte Flags { get; set; }

	public byte DesiredTemp_b0 { get; set; }

	public byte DesiredTemp_b1 { get; set; }

	public byte[] GetBytes()
	{
		return new byte[5] { Mode, FanMode, Flags, DesiredTemp_b0, DesiredTemp_b1 };
	}
}
// TODO: HACommand_HVACCommand
public class HACommand_HVACCommand
{
	public byte Command { get; set; }

	public byte GetBytes()
	{
		return Command;
	}
}


// TODO: HACommand_StatusLight
public class HACommand_StatusLight
{
	public List<EndpointValue> EndpointValues { get; private set; }

	internal void SetBytes(byte[] data, int headerOffset)
	{
		byte num = data[headerOffset];
		byte b = data[headerOffset + 1];
		if (EndpointValues == null)
		{
			EndpointValues = new List<EndpointValue>();
		}
		int num2 = num & 1;
		EndpointValue endpointValue = EndpointValues.Find((EndpointValue v) => v.ValueType == EndpointValueType.IsOn);
		if (endpointValue == null)
		{
			endpointValue = new EndpointValue();
			endpointValue.ValueType = EndpointValueType.IsOn;
			endpointValue.Value = num2.ToString();
			EndpointValues.Add(endpointValue);
		}
		else
		{
			endpointValue.Value = num2.ToString();
		}
		EndpointValue endpointValue2 = EndpointValues.Find((EndpointValue v) => v.ValueType == EndpointValueType.Dim);
		if (endpointValue2 == null)
		{
			endpointValue2 = new EndpointValue();
			endpointValue2.ValueType = EndpointValueType.Dim;
			endpointValue2.Value = b.ToString();
			EndpointValues.Add(endpointValue2);
		}
		else
		{
			endpointValue2.Value = b.ToString();
		}
	}
}
// TODO: HACommand_StatusHVAC
public class HACommand_StatusHVAC
{
	public List<EndpointValue> EndpointValues { get; private set; }

	internal void SetBytes(byte[] data, int headerOffset)
	{
		byte b = data[headerOffset];
		byte b2 = data[headerOffset + 1];
		byte b3 = data[headerOffset + 2];
		double value = Convert.ToDouble((double)(short)ByteArraysHelper.Portable16ToWORD(data[headerOffset + 3], data[headerOffset + 4]) / 100.0);
		double value2 = Convert.ToDouble((double)(short)ByteArraysHelper.Portable16ToWORD(data[headerOffset + 5], data[headerOffset + 6]) / 100.0);
		int num = ByteArraysHelper.Portable16ToWORD(data[headerOffset + 7], data[headerOffset + 8]);
		int num2 = ByteArraysHelper.Portable16ToWORD(data[headerOffset + 9], data[headerOffset + 10]);
		if (EndpointValues == null)
		{
			EndpointValues = new List<EndpointValue>();
		}
		EndpointValue endpointValue = EndpointValues.Find((EndpointValue v) => v.ValueType == EndpointValueType.ThermostatMode);
		if (endpointValue == null)
		{
			endpointValue = new EndpointValue();
			endpointValue.ValueType = EndpointValueType.ThermostatMode;
			endpointValue.Value = b.ToString();
			EndpointValues.Add(endpointValue);
		}
		else
		{
			endpointValue.Value = b.ToString();
		}
		EndpointValue endpointValue2 = EndpointValues.Find((EndpointValue v) => v.ValueType == EndpointValueType.ThermostatFanMode);
		if (endpointValue2 == null)
		{
			endpointValue2 = new EndpointValue();
			endpointValue2.ValueType = EndpointValueType.ThermostatFanMode;
			endpointValue2.Value = b2.ToString();
			EndpointValues.Add(endpointValue2);
		}
		else
		{
			endpointValue2.Value = b2.ToString();
		}
		EndpointValue endpointValue3 = EndpointValues.Find((EndpointValue v) => v.ValueType == EndpointValueType.MeasuredTemperatureC);
		if (endpointValue3 == null)
		{
			endpointValue3 = new EndpointValue();
			endpointValue3.ValueType = EndpointValueType.MeasuredTemperatureC;
			endpointValue3.Value = FloatHelper.DoubleToString(value);
			EndpointValues.Add(endpointValue3);
		}
		else
		{
			endpointValue3.Value = FloatHelper.DoubleToString(value);
		}
		EndpointValue endpointValue4 = EndpointValues.Find((EndpointValue v) => v.ValueType == EndpointValueType.ThermostatDesiredTempC);
		if (endpointValue4 == null)
		{
			endpointValue4 = new EndpointValue();
			endpointValue4.ValueType = EndpointValueType.ThermostatDesiredTempC;
			endpointValue4.Value = FloatHelper.DoubleToString(value2);
			EndpointValues.Add(endpointValue4);
		}
		else
		{
			endpointValue4.Value = FloatHelper.DoubleToString(value2);
		}
		EndpointValue endpointValue5 = EndpointValues.Find((EndpointValue v) => v.ValueType == EndpointValueType.HVACFlags);
		if (endpointValue5 == null)
		{
			endpointValue5 = new EndpointValue();
			endpointValue5.ValueType = EndpointValueType.HVACFlags;
			endpointValue5.Value = b3.ToString();
			EndpointValues.Add(endpointValue5);
		}
		else
		{
			endpointValue5.Value = b3.ToString();
		}
		EndpointValue endpointValue6 = EndpointValues.Find((EndpointValue v) => v.ValueType == EndpointValueType.HVACTimerOnMinutes);
		if (endpointValue6 == null)
		{
			endpointValue6 = new EndpointValue();
			endpointValue6.ValueType = EndpointValueType.HVACTimerOnMinutes;
			endpointValue6.Value = num.ToString();
			EndpointValues.Add(endpointValue6);
		}
		else
		{
			endpointValue6.Value = num.ToString();
		}
		EndpointValue endpointValue7 = EndpointValues.Find((EndpointValue v) => v.ValueType == EndpointValueType.HVACTimerOffMinutes);
		if (endpointValue7 == null)
		{
			endpointValue7 = new EndpointValue();
			endpointValue7.ValueType = EndpointValueType.HVACTimerOffMinutes;
			endpointValue7.Value = num2.ToString();
			EndpointValues.Add(endpointValue7);
		}
		else
		{
			endpointValue7.Value = num2.ToString();
		}
	}
}
// TODO: HACommand_Status
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


// TODO: FloatHelper
public static class FloatHelper
{
	public static double StringToDouble(string value)
	{
		return double.Parse(value, CultureInfo.InvariantCulture);
	}

	public static string DoubleToString(double value)
	{
		if (value == 0.0)
		{
			return "0";
		}
		string text = value.ToString("F");
		text = text.Replace(",", ".");
		string[] array = text.Split(new char[1] { '.' });
		if (array != null && array.Length > 1)
		{
			text = array[0] + "." + array[1].Substring(0, 1);
		}
		return text.Replace(",", ".");
	}

	public static string FormatTemperatureToString(double value, int numberOfDecimals)
	{
		if (value <= -300.0)
		{
			return "0";// StringResources.Instance.Devices.Thermostat_UnknownTemperature;
		}
		string text = value.ToString("F");
		text = text.Replace(",", ".");
		string[] array = text.Split(new char[1] { '.' });
		if (array != null && array.Length > 1)
		{
			text = ((numberOfDecimals <= 0) ? array[0] : (array[0] + "." + array[1].Substring(0, numberOfDecimals)));
		}
		return text + " °C";
	}
}

// TODO: Main
class MainClass
{
   public static HACommand Send_Cmd_Ping()
   {
      HACommand command = new HACommand();
      command.Header.ProtocolVersion = 0;
      command.Header.SourceMAC = new byte[6];
      command.Header.DestinationMAC = new byte[6] { 255, 255, 255, 255, 255, 255 };
      Random RandNumber = new Random();
      byte NextSequenceNumberSent = Convert.ToByte(RandNumber.Next(256));
      command.Header.SequenceNumber = NextSequenceNumberSent;
      command.Header.SourceEndpoint = 0;
      command.Header.DestinationEndpoint = 0;
      command.Header.CommandID = 224;
      command.Ping = new HACommand_Ping();
      return command;
   }

   private static HACommand ProcessReceivedMessage(byte[] data, string remoteAddress)
		{
         var DeviceIPInfo = new List<DeviceIPInformation>();
         var EndpointIPInfo = new List<EndpointIPInformation>();
         HACommand responseCommand = new HACommand();
			if (data == null || string.IsNullOrWhiteSpace(remoteAddress))
			{
				return null;
			}
			responseCommand.SetBytes(data, 0);
			
			DeviceIPInformation deviceIPInformation = null;
			string SourceMac = ByteArraysHelper.MacAddressBytesToString(responseCommand.Header.SourceMAC);
			string SourceEndpoint = responseCommand.Header.SourceEndpoint.ToString();
			deviceIPInformation = DeviceIPInfo.Find((DeviceIPInformation d) => d.DeviceMac == SourceMac);
			if (deviceIPInformation != null)
			{
				if (string.IsNullOrWhiteSpace(deviceIPInformation.LocalIPAddress))
				{
					//Send_Cmd_Subscribe(responseCommand.Header.SourceMAC, remoteAddress);
				}
				deviceIPInformation.LocalIPAddress = remoteAddress;
			}
      
			return responseCommand;
		}
   
   public static HACommand Send_Cmd_Join_Scan()
		{
			HACommand command = new HACommand();
			command.Header.ProtocolVersion = 0;
			command.Header.SourceMAC = new byte[6];
			command.Header.DestinationMAC = mac;
			Random RandNumber = new Random();
         byte NextSequenceNumberSent = Convert.ToByte(RandNumber.Next(256));
         command.Header.SequenceNumber = NextSequenceNumberSent;
			command.Header.SourceEndpoint = 0;
			command.Header.DestinationEndpoint = 0;
			command.Header.CommandID = 161;
			if (command.Join == null)
			{
				command.Join = new HACommand_Join();
			}
			command.Join.SubCommand = 5;
			
			return command;
		}

      public static HACommand Send_Cmd_Join_JoinRequest(byte[] sSID, byte securityType, byte encryptionType, byte[] key)
		{
         HACommand command = new HACommand();
			command.Header.ProtocolVersion = 0;
			command.Header.SourceMAC = new byte[6];
			command.Header.DestinationMAC = mac;
			Random RandNumber = new Random();
         byte NextSequenceNumberSent = Convert.ToByte(RandNumber.Next(256));
         command.Header.SequenceNumber = NextSequenceNumberSent;
			command.Header.SourceEndpoint = 0;
			command.Header.DestinationEndpoint = 0;
			command.Header.CommandID = 161;
         if (command.Join == null)
			{
				command.Join = new HACommand_Join();
			}
			if (command.Join.JoinRequest == null)
			{
				command.Join.JoinRequest = new HACommand_Join_JoinRequest(sSID, securityType, encryptionType, key);
			}
			command.Join.SubCommand = 3;
         return command;
			
		}


   public static HACommand SendCustomCommand(byte cmdID, byte[] cmdParams, byte endpointAddress)
		{
         HACommand command =new HACommand();
			command.Header.ProtocolVersion = 0;
			command.Header.SourceMAC = new byte[6];
			command.Header.DestinationMAC = mac;
			Random RandNumber = new Random();
         byte NextSequenceNumberSent = Convert.ToByte(RandNumber.Next(256));
         command.Header.SequenceNumber = NextSequenceNumberSent;
			command.Header.SourceEndpoint = 0;
			command.Header.DestinationEndpoint = endpointAddress;
			command.Header.CommandID = 2;
			
			return command;
			
		}

   static byte[] mac;
   static byte[] SSID;
   static byte SecurityType;
   static byte EncryptionType;
   public static void Main()
   {
      /*CONECTAR A WIFI*/
	  /*
      IPEndPoint lep = new IPEndPoint(IPAddress.Any, 20910);    
      IPEndPoint rep = new IPEndPoint(IPAddress.Parse("10.10.100.254"), 20910);
      
      HACommand command = Send_Cmd_Ping();
      byte[] data = command.GetBytes();
      UdpClient client = new UdpClient();

      client.ExclusiveAddressUse = false;
      client.Client.SetSocketOption(SocketOptionLevel.Socket, SocketOptionName.ReuseAddress, true);
      client.Client.Bind(lep);
      client.BeginReceive(DataReceived, client);
      client.Send(data, data.Length, rep);
      Console.WriteLine(BitConverter.ToString(data));  
      Thread.Sleep(2050); 
 
      HACommand commandScan = Send_Cmd_Join_Scan();
      byte[] dataScan = commandScan.GetBytes();
      client.BeginReceive(DataReceivedScan, client);
      client.Send(dataScan, dataScan.Length, rep);
      Console.WriteLine(BitConverter.ToString(dataScan));  
      Thread.Sleep(15050); 
     
      Console.WriteLine(Encoding.UTF8.GetString(SSID,0,SSID.Length));
      Console.WriteLine(EncryptionType);
      Console.WriteLine(SecurityType);

      byte[] bytes2 = Encoding.UTF8.GetBytes("j2LK98!we");
      
      HACommand commandJoin = Send_Cmd_Join_JoinRequest(SSID,SecurityType,EncryptionType,bytes2);
      byte[] dataJoin = commandJoin.GetBytes();
      client.BeginReceive(DataReceivedJoin, client);
      client.Send(dataJoin, dataJoin.Length, rep);
      Console.WriteLine(BitConverter.ToString(dataJoin));  
      Thread.Sleep(2050); */
      /*FINAL CONECTAR A WIFI*/

      /*ENCENDER*/
	  
      IPEndPoint lep = new IPEndPoint(IPAddress.Any, 20910);    
      IPEndPoint rep = new IPEndPoint(IPAddress.Parse("172.16.4.3"), 20910);
      
      HACommand command = Send_Cmd_Ping();
      byte[] data = command.GetBytes();
      UdpClient client = new UdpClient();

      client.ExclusiveAddressUse = false;
      client.Client.SetSocketOption(SocketOptionLevel.Socket, SocketOptionName.ReuseAddress, true);
      client.Client.Bind(lep);
      client.BeginReceive(DataReceived, client);
      client.Send(data, data.Length, rep);
      Console.WriteLine(BitConverter.ToString(data));  
      Thread.Sleep(2050); 

      HACommand commandCustom = SendCustomCommand(1, null, 1);
      byte[] dataCustom = commandCustom.GetBytes();
      client.BeginReceive(DataReceived, client);
      client.Send(dataCustom, dataCustom.Length, rep);
      Console.WriteLine(BitConverter.ToString(dataCustom));  
      Thread.Sleep(2050); 
   }

   static void DataReceived(IAsyncResult ar)
   {
      UdpClient c = (UdpClient)ar.AsyncState;
      IPEndPoint receivedIpEndPoint = new IPEndPoint(IPAddress.Any,0);
      Byte[] receivedBytes = c.EndReceive(ar, ref receivedIpEndPoint); 
      Console.WriteLine(BitConverter.ToString(receivedBytes));    
      HACommand responseCommand = ProcessReceivedMessage(receivedBytes,"10.10.100.254");
      Console.WriteLine("dasdasd");
	  Console.WriteLine(ByteArraysHelper.MacAddressBytesToString(responseCommand.Header.SourceMAC));
      mac=responseCommand.Header.SourceMAC;
      
   }

   static void DataReceivedScan(IAsyncResult ar)
   {
      UdpClient c = (UdpClient)ar.AsyncState;
      IPEndPoint receivedIpEndPoint = new IPEndPoint(IPAddress.Any, 0);
      Byte[] receivedBytes = c.EndReceive(ar, ref receivedIpEndPoint); 
      c.BeginReceive(DataReceivedScan, c);
      Console.WriteLine(BitConverter.ToString(receivedBytes));    
      HACommand responseCommand = new HACommand();
      responseCommand.SetBytes(receivedBytes, 6);
      if (responseCommand.Join != null && responseCommand.Join.ScanResponse != null && responseCommand.Join.ScanResponse.ListOfWifis != null)
      {
         foreach(HACommand_Join_ScanResponseItem item in responseCommand.Join.ScanResponse.ListOfWifis){
               string ssid=Encoding.Default.GetString(item.SSID,0,item.SSID.Length);
               
			   if (ssid[0]=='I' && ssid[1]=='P' && ssid[2]=='M' && ssid[3]=='L'){
                  SSID=item.SSID;
                  SecurityType=item.SecurityType;
                  EncryptionType=item.EncryptionType;
				  Console.WriteLine(ssid);    
                  break;
               }
         }
         
      }
   }
  
   static void DataReceivedJoin(IAsyncResult ar)
   {
      UdpClient c = (UdpClient)ar.AsyncState;
      IPEndPoint receivedIpEndPoint = new IPEndPoint(IPAddress.Any, 0);
      Byte[] receivedBytes = c.EndReceive(ar, ref receivedIpEndPoint); 
      c.BeginReceive(DataReceivedJoin, c);
      Console.WriteLine(BitConverter.ToString(receivedBytes));    
      HACommand responseCommand = new HACommand();
		if (receivedBytes != null && receivedBytes.Length == 17 && receivedBytes[16] == 128)
      {
         Console.WriteLine("OK");
         return;
      }

   }
}