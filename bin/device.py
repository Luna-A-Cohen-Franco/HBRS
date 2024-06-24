from typing import Tuple
import socket
import random
import os
import sys

from scapy.all import * # type: ignore

from lib.classes.addresses.ipv4 import IPv4Addr
from lib.classes.hacommand.ping import Ping
from lib.classes.join.join_req import JoinReq
from lib.consts.other import Other
sys.path.append(os.path.dirname(__file__) + "/..")

from lib.classes.addresses.mac import MacAddress
from lib.classes.hacommand.hacommand import HACommand
from lib.classes.join.join import Join

NIPREP_PORT = 20911 # Might be 20910

class Device:
    def __init__(self, mac: str, ssid: str, security_type: int, encryption_type: int, deviceSckt: socket.socket, nipDeviceSckt):
        self.mac = mac
        self.ssid = ssid
        self.security_type = security_type
        self.encryption_type = encryption_type
        self.ip = "0.0.0.0"
        self.deviceSckt = deviceSckt
        self.nipDeviceSckt = nipDeviceSckt
        self.connected = False
        self.got_mac = False

    def process_rcvd_msg(self, data: list):
        res = HACommand()

        res.set_bytes(data, 0)

        return res

    def data_received(self) -> Tuple[MacAddress, str]:
        buf = bytearray(1024) 
        while True:
            try:
                size, _ = self.deviceSckt.recvfrom_into(buf)
                print("Recieved ARP data")
                self.got_mac = True
                received_bytes = buf[:size]
                res = self.process_rcvd_msg(received_bytes)

                if res:
                    res.get_header().get_source_mac().display()
                    return res.get_header().get_source_mac(), None
                else:
                    return None, "Invalid data"
            except Exception as e:
                return None, str(e)
        

    def data_received_join(self) -> Tuple[None, str]:
        buffer = bytearray(1024)
        while True:
            try:
                size, _ = self.deviceSckt.recvfrom_into(buffer)
                print("Received join bytes")
                received_bytes = buffer[:size]
                print(received_bytes)

                if len(received_bytes) == 17 and received_bytes[16] == 128:
                    print("OK")
                    return None, None
                else:
                    return None, "Invalid data"

            except Exception as e:
                return None, str(e)
    
    def data_received_scan(self, ssid: str) -> Tuple[None, str]:
        buffer = bytearray(1024)
        
        while True:
            try:
                size, _ = self.deviceSckt.recvfrom_into(buffer)
                received_bytes = buffer[:size]
                print("Recieved scan data")
                print(received_bytes)

                res = HACommand()
                res.set_bytes(received_bytes, 6)
                
                if (
                    res.get_join() is None
                    or res.get_join().get_scan_res() is None
                    or not res.get_join().get_scan_res().get_wifis()
                ):
                    print("Data wasn't a scan response or no wifis were found")
                    return None, "Data wasn't a scan response or no wifis were found"
                
                for wifi in res.get_join().get_scan_res().get_wifis():
                    print("WIFI: ")
                    wifi.display()

                    if wifi.get_ssid_as_str().startswith(ssid):
                        self.ssid = wifi.ssid
                        self.security_type = wifi.get_security_type()
                        self.encryption_type = wifi.get_encryption_type()
                        
                        print("Selected WIFI:")
                        self.display()
                        
                        self.connected = True
                    print()
            except Exception as e:
                return None, str(e)
    
    def find_new_ip(self):
        while True:
            data, addr = self.nipDeviceSckt.recvfrom(1024)
            print("Recieved new IP data")
            print("Received message:", data, "from", addr)
            sender_ip = addr[0]
                    
            sender_mac = self.get_mac_address_from_ip(sender_ip)

            #TODO: Make this a valid comparison
            if sender_mac == self.mac.get_bytes():
                print("Found IP")
                self.ip = sender_ip # TODO: Check that this is giving a string, not sure if its a string, an int or an array of bytes
                return
    
    def get_mac_address_from_ip(self, ip):
        ans, _ = srp(Ether(dst="ff:ff:ff:ff:ff:ff")/ARP(pdst=ip), timeout=2, verbose=False) # type: ignore
        
        # Extract MAC address from response
        for _, rcv in ans:
            return rcv[Ether].src # type: ignore

    def get_niprep(self)  -> Tuple[str, int]:
        self.find_new_ip()

        return (self.ip, NIPREP_PORT)

    def display(self):
        print(''.join(chr(byte) for byte in self.ssid))
        print(self.ssid)
        print(self.security_type)
        print(self.encryption_type)