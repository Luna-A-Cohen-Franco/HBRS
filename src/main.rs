use hbrs::classes::{ha_comm::{ha_command::HACommand, ping::Ping}, addresses::mac_address::MacAddress, join::{join::Join, join_req::JoinReq}, device_ip_info::DeviceIPInfo, endpoint::endpoint_ip_info::EndpointIPInformation};
use rand::Rng;
use std::net::{UdpSocket, SocketAddr};

struct Runner{
    mac: MacAddress,
    ssid: Vec<u8>,
    security_type: u8,
    encryption_type: u8
}

impl Runner{
    fn send_ping(&self) -> HACommand{
        let mut comm = HACommand::new();
        *comm.get_header_mut().get_protocol_version_mut() = 0;
        *comm.get_header_mut().get_source_mac_mut() = MacAddress::new(vec![0, 0, 0, 0, 0, 0]).unwrap();
        *comm.get_header_mut().get_destination_mac_mut() = MacAddress::new(vec![255, 255, 255, 255, 255, 255]).unwrap();
    
        let mut rng = rand::thread_rng();
        let next_sqnc_num: u8 = rng.gen_range(0..=255);
        *comm.get_header_mut().get_sequence_number_mut() = next_sqnc_num;
    
        *comm.get_header_mut().get_source_endpoint_mut() = 0;
        *comm.get_header_mut().get_destination_endpoint_mut() = 0;
        *comm.get_header_mut().get_id_mut() = 224;
        comm.set_ping(Some(Ping::new()));
    
        return comm;
    }
    
    fn send_join_scan(&self) -> HACommand{
        let mut comm = HACommand::new();
        *comm.get_header_mut().get_protocol_version_mut() = 0;
        *comm.get_header_mut().get_source_mac_mut() = MacAddress::new(vec![0, 0, 0, 0, 0, 0]).unwrap();
        *comm.get_header_mut().get_destination_mac_mut() = self.mac.clone();
        
        let mut rng = rand::thread_rng();
        let next_sqnc_num: u8 = rng.gen_range(0..=255);
        *comm.get_header_mut().get_sequence_number_mut() = next_sqnc_num;
    
        *comm.get_header_mut().get_source_endpoint_mut() = 0;
        *comm.get_header_mut().get_destination_endpoint_mut() = 0;
        *comm.get_header_mut().get_id_mut() = 161;
    
        comm.set_join(Some(Join::new()));
    
        return comm
    }
    
    fn send_join_request(&self, ssid: Vec<u8>, security_type: u8, encryption_type: u8, key: Vec<u8>) -> HACommand{
        let mut comm = HACommand::new();
    
        *comm.get_header_mut().get_protocol_version_mut() = 0;
        *comm.get_header_mut().get_source_mac_mut() = MacAddress::new(vec![0, 0, 0, 0, 0, 0]).unwrap();
        *comm.get_header_mut().get_destination_mac_mut() = self.mac.clone();
        
        let mut rng = rand::thread_rng();
        let next_sqnc_num: u8 = rng.gen_range(0..=255);
        *comm.get_header_mut().get_sequence_number_mut() = next_sqnc_num;
    
        *comm.get_header_mut().get_source_endpoint_mut() = 0;
        *comm.get_header_mut().get_destination_endpoint_mut() = 0;
        *comm.get_header_mut().get_id_mut() = 161;
    
        let mut join = Join::new();
    
        match JoinReq::new(ssid, security_type, encryption_type, key){
            Ok(req) => join.set_join_req(Some(req)),
            Err(err) => print!("{:?}", err),
        }
    
        *join.get_sub_command_mut() = 3;
    
        comm.set_join(Some(join));
    
        return comm;
    }
    
    fn send_custom_comm(&self, dest_endpoint: u8) -> HACommand{
        let mut comm = HACommand::new();
    
        *comm.get_header_mut().get_protocol_version_mut() = 0;
        *comm.get_header_mut().get_source_mac_mut() = MacAddress::new(vec![0, 0, 0, 0, 0, 0]).unwrap();
        *comm.get_header_mut().get_destination_mac_mut() = self.mac.clone();
        
        let mut rng = rand::thread_rng();
        let next_sqnc_num: u8 = rng.gen_range(0..=255);
        *comm.get_header_mut().get_sequence_number_mut() = next_sqnc_num;
    
        *comm.get_header_mut().get_source_endpoint_mut() = 0;
        *comm.get_header_mut().get_destination_endpoint_mut() = dest_endpoint;
        *comm.get_header_mut().get_id_mut() = 2;

        return comm;
    }
    
    fn process_rcvd_msg(&self, data: &mut Vec<u8>) -> Result<HACommand, String>{
        let mut res: HACommand = HACommand::new();

        if data.is_empty(){
            return Err(String::from("Data is empty"));
        }

        res.set_bytes(data, 0);

        return Ok(res);
    }

    fn data_recieved(&self) -> Result<HACommand, String>{
        let socket = match UdpSocket::bind("0.0.0.0:0"){
            Ok(sckt) => sckt,
            Err(_) => return Err(String::from("Couldnt bind to address 0.0.0.0:0"))
        };

        let mut buffer = [0u8; 1024];
        let size = match socket.recv_from(&mut buffer){
            Ok((size, _)) => size,
            Err(_) => return Err(String::from("Timed out")),
        };
        let received_bytes = &buffer[..size];
        
        println!("{:X?}", received_bytes);
        
        match self.process_rcvd_msg(&mut received_bytes.to_vec()){
            Ok(res) => return Ok(res),
            Err(_) => Err(String::from("Invalid data"))
        }
    }

    fn data_recieved_join(&self) -> Result<(), String>{
        let socket = match UdpSocket::bind("0.0.0.0:0"){
            Ok(sckt) => sckt,
            Err(_) => return Err(String::from("Couldnt bind to address 0.0.0.0:0"))
        };

        let mut buffer = [0u8; 1024];
        let (size, _) = match socket.recv_from(&mut buffer){
            Ok((size, _)) => (size, _),
            Err(_) => return Err(String::from("Timed out")),
        };

        let received_bytes = &buffer[..size];

        println!("{:X?}", received_bytes);

        if received_bytes.len() == 17 && received_bytes[16] == 128 {
            return Ok(());
        }

        return Err(String::from("Recieved messeage didnt meet the boundaries"));
    }
    /*
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
     */
}
fn main(){

}
