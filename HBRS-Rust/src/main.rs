use hbrs::classes::{ha_comm::{ha_command::HACommand, ping::Ping}, addresses::mac_address::MacAddress, join::{join::Join, join_req::JoinReq}};
use rand::Rng;
use std::{net::{SocketAddr, UdpSocket}, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread};

#[derive(Debug, Clone)]
struct Runner{
    pub mac: MacAddress,
    pub ssid: Vec<u8>,
    pub security_type: u8,
    pub encryption_type: u8
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

    fn data_recieved(&self, socket: Arc<UdpSocket>, running: Arc<AtomicBool>) -> Result<HACommand, String>{
        let mut buf = [0; 1024]; // Adjust buffer size as needed
        while running.load(Ordering::Relaxed) {
            match socket.recv_from(&mut buf) {
                Ok((size, _)) => {
                    let received_bytes = &buf[..size];

                    let response_command = self.process_rcvd_msg(&mut received_bytes.to_vec());

                    match response_command{
                        Ok(res) => {
                            res.get_header_ref().get_source_mac_ref().display();
                            return Ok(res);
                        },
                        Err(_) => return Err(String::from("Invalid data")),
                    }
                }
                Err(_) =>  return Err(String::from("Invalid data")),
            }
        }
        return Err(String::from("Invalid data"));
    }

    fn data_recieved_join(&self, socket: Arc<UdpSocket>, running: Arc<AtomicBool>) -> Result<(), String>{
        let mut buffer = [0u8; 1024];
        while running.load(Ordering::Relaxed) {
            match socket.recv_from(&mut buffer) {
                Ok((size, _)) => {
                    let received_bytes = &buffer[..size];

                    match String::from_utf8(received_bytes.to_vec()){
                        Ok(s) => println!("{}", s),
                        Err(_) => {return Err(String::from("Invalid data"))}
                    }

                    if received_bytes.len() == 17 && received_bytes[16] == 128 {
                        return Ok(());
                    }
                }
                Err(_) =>  return Err(String::from("Invalid data")),
            }
        }
        return Err(String::from("Recieved messeage didnt meet the boundaries"));
    }

    fn data_recieved_scan(&mut self, socket: Arc<UdpSocket>, running: Arc<AtomicBool>) -> Result<(), String>{
        let mut buffer = [0u8; 1024];
        while running.load(Ordering::Relaxed) {
            match socket.recv_from(&mut buffer) {
                Ok((size, _)) => {
                    let received_bytes = &buffer[..size];

                    let mut res = HACommand::new();
            
                    res.set_bytes(&mut received_bytes.to_vec(), 6);
            
                    if res.get_join().is_none()  || 
                        res.get_join().unwrap().get_scan_res().is_none() || 
                        res.get_join().unwrap().get_scan_res().unwrap().get_wifis_ref().is_empty(){
                        return Err(String::from("Data wasn't a scan response or no wifis were found"));
                    }
            
                    'inner: for wifi in res.get_join().unwrap().get_scan_res().unwrap().get_wifis_ref(){
                        let ssid = wifi.get_ssid_as_str().unwrap();
                        if ssid.chars().nth(0) == Some('I') &&
                            ssid.chars().nth(1) == Some('P') &&
                            ssid.chars().nth(2) == Some('M') &&
                            ssid.chars().nth(3) == Some('L') {
                            self.ssid = wifi.get_ssid_ref().clone();
                            self.security_type = *wifi.get_security_type_ref();
                            self.encryption_type = *wifi.get_encryption_type_ref();
                            
                            print!("{}", ssid);
                            break 'inner;
                        }
                    }

                    return Ok(())
                }
                Err(_) =>  return Err(String::from("Invalid data")),
            }
        }       
        return Err(String::from("Invalid data"))
    }
}

fn data_recieved(runner: &Arc<Mutex<Runner>>, client: &Arc<UdpSocket>, running: &Arc<AtomicBool>, rep: &SocketAddr){
    let command = runner.lock().unwrap().send_ping();
    let data = command.get_bytes();

    let client_clone = client.clone();
    let running_clone = running.clone();
    let runner_clone = Arc::clone(&runner);

    let _ = thread::spawn(move || {
        print!("Recieving");

        runner_clone.lock().unwrap().data_recieved(client_clone, running_clone).unwrap();
    });

    client.send_to(&data, rep).unwrap();

    thread::sleep(std::time::Duration::from_secs(2));
}

fn data_scan(runner: &Arc<Mutex<Runner>>, client: &Arc<UdpSocket>, running: &Arc<AtomicBool>, rep: &SocketAddr){
    let command_scan = runner.lock().unwrap().send_join_scan();
    let data_scan = command_scan.get_bytes();

    let client_clone = client.clone();
    let running_clone = running.clone();
    let runner_clone = Arc::clone(&runner);

    let _ = thread::spawn(move || {
        print!("Scanning");

        runner_clone.lock().unwrap().data_recieved_scan(client_clone, running_clone).unwrap();
    });
    
    client.send_to(&data_scan, rep).unwrap();

    thread::sleep(std::time::Duration::from_secs(15));
}

fn data_join(runner: &Arc<Mutex<Runner>>, client: &Arc<UdpSocket>, running: &Arc<AtomicBool>, rep: &SocketAddr){
    let bytes2 = "j2LK98!we".as_bytes();

    let command_join = runner.lock().unwrap().send_join_request(runner.lock().unwrap().ssid.clone(), runner.lock().unwrap().security_type, runner.lock().unwrap().encryption_type, bytes2.to_vec());

    let data_join = command_join.get_bytes();

    let client_clone = client.clone();
    let running_clone = running.clone();
    let runner_clone = Arc::clone(&runner);

    let _ = thread::spawn(move || {
        print!("Joining");

        runner_clone.lock().unwrap().data_recieved_join(client_clone, running_clone).unwrap();
    });

    client.send_to(&data_join, rep).unwrap();

    thread::sleep(std::time::Duration::from_secs(2));
}

fn data_custom(runner: &Arc<Mutex<Runner>>, client: &Arc<UdpSocket>, rep: &SocketAddr){
    let custom_comm = runner.lock().unwrap().send_custom_comm(1);

    let data_custom = custom_comm.get_bytes();

    print!("Sending custom signal");

    client.send_to(&data_custom, rep).unwrap();

    thread::sleep(std::time::Duration::from_secs(2));
}

fn main(){
    let runner = Arc::new(Mutex::new(Runner{
        mac: MacAddress::new(vec![0, 0, 0, 0, 0, 0]).unwrap(),
        ssid: Vec::new(),
        security_type: 0,
        encryption_type: 0
    }));

    let lep = SocketAddr::from(([0, 0, 0, 0], 20910));
    let rep = SocketAddr::from(([10, 10, 100, 254], 20910));

    let client = Arc::new(UdpSocket::bind(lep).unwrap());
    let running = Arc::new(AtomicBool::new(true));

    data_recieved(&runner, &client, &running, &rep);
    data_scan(&runner, &client, &running, &rep);
    
    print!("{:?}", String::from_utf8(runner.lock().unwrap().ssid.clone()));
    print!("{:?}", runner.lock().unwrap().security_type);
    print!("{:?}", runner.lock().unwrap().encryption_type);

    data_join(&runner, &client, &running, &rep);

    data_custom(&runner, &client, &rep);
}