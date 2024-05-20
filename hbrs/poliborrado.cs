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
}