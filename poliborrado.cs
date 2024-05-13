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