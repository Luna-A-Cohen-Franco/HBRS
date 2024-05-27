from typing import Tuple
import socket
import random
import os
import sys

from lib.classes.hacommand.ping import Ping
from lib.classes.join.join_req import JoinReq
sys.path.append(os.path.dirname(__file__) + "/..")

from lib.classes.addresses.mac import MacAddress
from lib.classes.hacommand.hacommand import HACommand
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

        comm.ping = Ping();  
        print(comm.get_header().get_protocol_version())

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

        join.set_join_req(JoinReq(ssid, security_type, encryption_type, key))

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
        buf = bytearray(1024) 
        while True:
            try:
                size, _ = socket.recvfrom_into(buf)
                print(size)
                print("Recieved data")
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
                        
                        self.display()

                        return None, None
            except Exception as e:
                return None, str(e)
            
    def display(self):
        print(self.ssid)
        print(self.security_type)
        print(self.encryption_type)