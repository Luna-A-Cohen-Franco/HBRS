from enum import Enum
from threading import Thread
from concurrent.futures import ThreadPoolExecutor
import time
import socket
import os
import sys
sys.path.append(os.path.dirname(__file__) + "/..")

from bin.device import Device
from bin.comm_gen import CommandGenerator
from lib.classes.addresses.mac import MacAddress

#Consts
NIPREP_PORT = 20911
LEP = ('0.0.0.0', 20910)
REP = ('10.10.100.254', 20910)
NIPEP = ('0.0.0.0', 20911)

class DeviceManager:
    def __init__(self, name, ssid, key):
        self.name = name

        self.ssid = ssid
        self.key = key.encode()

        self.device = DeviceManager.gen_device()
        
        self.deviceSckt = DeviceManager.gen_device_sckt()
        self.nipDeviceSckt = DeviceManager.gen_nip_device_sck()

        self.niprep = ""

        self.device.deviceSckt = self.deviceSckt
        self.device.nipDeviceSckt = self.nipDeviceSckt
        
    @staticmethod
    def gen_device_sckt():
        deviceSckt = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        deviceSckt.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)  
        deviceSckt.bind(LEP)

        return deviceSckt

    @staticmethod
    def gen_nip_device_sck():
        nipDeviceSckt = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        nipDeviceSckt.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)  
        nipDeviceSckt.bind(NIPEP)

        return nipDeviceSckt

    @staticmethod
    def gen_device():
        return Device(
            mac=MacAddress([0, 0, 0, 0, 0, 0]),
            ssid=[],
            security_type=0,
            encryption_type=0,
            deviceSckt = ("", 0),
            nipDeviceSckt = ("", 0)
        )
    
    def data_received(self):
        command = CommandGenerator.send_ping()
        ping = command.get_bytes()

        def thread_func():
            print("Receiving")
            (mac, _) = self.device.data_received()
            self.device.mac = mac

        Thread(target=thread_func).start()

        while self.device.got_mac == False:
            self.deviceSckt.sendto(ping, REP)

        time.sleep(5)

    def data_scan(self):
        command_scan = CommandGenerator.send_join_scan(self.device.mac)
        data_scan = command_scan.get_bytes()

        def thread_func():
            print("Scanning")
            self.device.data_received_scan(self.ssid)

        Thread(target=thread_func).start()

        self.deviceSckt.sendto(data_scan, REP)

        while self.device.connected == False:
            time.sleep(15)
            
        time.sleep(15)

    def data_join(self):
        command_join = CommandGenerator.send_join_request(self.device.mac, self.device.ssid, self.device.security_type, self.device.encryption_type, self.key)

        data_join = command_join.get_bytes()

        print(list(data_join))
        def thread_func():
            print("Joining")
            time.sleep(5)
            self.device.data_received_join()

        Thread(target=thread_func).start()

        self.deviceSckt.sendto(data_join, REP)

        time.sleep(2)

    def data_custom(self):
        custom_comm = CommandGenerator.send_custom_comm(self.device.mac, 1)

        data_custom = custom_comm.get_bytes()

        print("Sending custom signal")

        self.deviceSckt.sendto(data_custom, REP)

        time.sleep(2)

    def data_find_new_ip(self):
        self.device.find_new_ip()


    def make_connection(self):
        self.data_received()

        self.data_scan()

        self.data_join()

        """nip = self.data_find_new_ip(self.device)

        self.niprep = (nip, NIPREP_PORT) # Might be 20910

        self.name = self.device.mac + " - " + self.name"""

def main():
    devMnger = DeviceManager(name = "AHH", ssid = "IPMLabo", key = "j2LK98!we")

    devMnger.make_connection()

main()