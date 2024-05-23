import socket
from typing import Tuple
from lib.classes.addresses.mac import MacAddress
from lib.classes.hacommand.hacommand import HACommand
import random

from lib.classes.join.join import Join


class Runner:
    def __init__(self, mac: str, ssid: str, security_type: int, encryption_type: int):
        self.mac = mac
        self.ssid = ssid
        self.security_type = security_type
        self.encryption_type = encryption_type

    def send_ping(self):
        comm = HACommand()
        comm.get_header().set_protocol_version(0)
        comm.get_header().set_source_mac(MacAddress([0, 0, 0, 0, 0, 0]))
        comm.get_header().set_destination_mac(MacAddress([255, 255, 255, 255, 255, 255]))

        rng = random.SystemRandom()
        next_sqnc_num = rng.randint(0, 255)
        comm.get_header().set_sequence_number(next_sqnc_num)

        comm.get_header().set_source_endpoint(0)
        comm.get_header().set_destination_endpoint(0)
        comm.get_header().set_id(224)

        return comm
        
    def send_join_scan(self):
        comm = HACommand()
        comm.get_header().set_protocol_version(0)
        comm.get_header().set_source_mac(MacAddress([0, 0, 0, 0, 0, 0]))
        comm.get_header().set_destination_mac(MacAddress(self.mac))

        rng = random.SystemRandom()
        next_sqnc_num = rng.randint(0, 255)
        comm.get_header().set_sequence_number(next_sqnc_num)

        comm.get_header().set_source_endpoint(0)
        comm.get_header().set_destination_endpoint(0)
        comm.get_header().set_id(161)

        comm.set_join(Join())

        return comm

    def send_join_request(self, ssid: str, security_type: int, encryption_type: int, key: str):
        comm = HACommand()

        comm.get_header().set_protocol_version(0)
        comm.get_header().set_source_mac(MacAddress([0, 0, 0, 0, 0, 0]))
        comm.get_header().set_destination_mac(MacAddress(self.mac))

        rng = random.SystemRandom()
        next_sqnc_num = rng.randint(0, 255)
        comm.get_header().set_sequence_number(next_sqnc_num)

        comm.get_header().set_source_endpoint(0)
        comm.get_header().set_destination_endpoint(0)
        comm.get_header().set_id(161)

        join = Join()

        join.set_join_req(ssid, security_type, encryption_type, key)

        join.set_sub_command(3)

        comm.set_join(join)

        return comm

    def send_custom_comm(self, dest_endpoint: int):
        comm = HACommand()

        comm.get_header().set_protocol_version(0)
        comm.get_header().set_source_mac(MacAddress([0, 0, 0, 0, 0, 0]))
        comm.get_header().set_destination_mac(MacAddress(self.mac))

        rng = random.SystemRandom()
        next_sqnc_num = rng.randint(0, 255)
        comm.get_header().set_sequence_number(next_sqnc_num)

        comm.get_header().set_source_endpoint(0)
        comm.get_header().set_destination_endpoint(dest_endpoint)
        comm.get_header().set_id(2)

        return comm

    def process_rcvd_msg(self, data: list):
        res = HACommand()

        if not data:
            return "Data is empty"

        res.set_bytes(data, 0)

        return res

    def data_received(self, socket: socket.socket) -> Tuple[HACommand, str]:
        buf = bytearray(1024)  # Adjust buffer size as needed
        while True:
            try:
                size, _ = socket.recvfrom_into(buf)
                received_bytes = buf[:size]
                response_command = self.process_rcvd_msg(received_bytes)
                if response_command:
                    response_command.get_header().get_source_mac().display()
                    return response_command, None
                else:
                    return None, "Invalid data"
            except Exception as e:
                return None, str(e)
        

    def data_received_join(self, socket: socket.socket) -> Tuple[None, str]:
        buffer = bytearray(1024)
        while True:
            try:
                size, _ = socket.recvfrom_into(buffer)
                received_bytes = buffer[:size]

                try:
                    received_str = received_bytes.decode('utf-8')
                    print(received_str)
                except UnicodeDecodeError:
                    return None, "Invalid data"

                if len(received_bytes) == 17 and received_bytes[16] == 128:
                    return None, None
            except Exception as e:
                return None, str(e)

    def data_received_scan(self, socket: socket.socket) -> Tuple[None, str]:
        buffer = bytearray(1024)
        
        while True:
            try:
                size, _ = socket.recvfrom_into(buffer)
                received_bytes = buffer[:size]

                res = HACommand()
                res.set_bytes(received_bytes, 6)

                if (
                    res.get_join() is None
                    or res.get_join().get_scan_res() is None
                    or not res.get_join().get_scan_res().get_wifis()
                ):
                    return None, "Data wasn't a scan response or no wifis were found"

                for wifi in res.get_join().get_scan_res().get_wifis():
                    ssid = wifi.get_ssid_as_str()
                    if ssid and ssid.startswith("IPML"):
                        self.ssid = wifi.get_ssid()
                        self.security_type = wifi.get_security_type()
                        self.encryption_type = wifi.get_encryption_type()
                        print(ssid)
                        return None, None
            except Exception as e:
                return None, str(e)
"""
impl Runner{    
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
}"""