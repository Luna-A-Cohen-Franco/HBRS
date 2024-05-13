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