import os
import sys
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from lib.classes.addresses.mac import MacAddress
from lib.consts.other import Other


class Header:
    def __init__(self):
        self.protocol_version = 0
        self.source_mac = MacAddress.new_empty()
        self.destination_mac = MacAddress.new_empty()
        self.sequence_number = 0
        self.source_endpoint = 0
        self.destination_endpoint = 0
        self.command_id = 0
        self.mac_of_last_response = MacAddress.new_empty()

    def get_bytes(self):
        data = [self.protocol_version]
        
        source_mac_bytes = self.source_mac.get_bytes()
        for b in source_mac_bytes:
            data.append(b)

        destination_mac_bytes = self.destination_mac.get_bytes()
        for b in destination_mac_bytes:
            data.append(b)

        data.append(self.sequence_number)
        data.append(self.source_endpoint)
        data.append(self.destination_endpoint)
        data.append(self.command_id)

        return data

    def set_bytes(self, data):
        self.protocol_version = data[0]

        self.source_mac.set_bytes(data[1:7])
        self.mac_of_last_response = self.source_mac

        self.sequence_number = data[13]
        self.source_endpoint = data[14]
        self.destination_endpoint = data[15]
        self.command_id = data[16]

    def clear_bytes(self):
        self.protocol_version = 0
        self.source_mac = MacAddress.new_empty()
        self.destination_mac = MacAddress.new_empty()
        self.sequence_number = 0
        self.source_endpoint = 0
        self.destination_endpoint = 0
        self.command_id = 0
        self.mac_of_last_response = MacAddress.new_empty()

    def get_id(self):
        return self.command_id
    
    def set_id(self, command_id):
        self.command_id = command_id
        
    def get_protocol_version(self):
        return self.protocol_version

    def set_protocol_version(self, protocol_version):
        self.protocol_version = protocol_version

    def get_source_mac(self):
        return self.source_mac

    def set_source_mac(self, source_mac):
        self.source_mac = source_mac

    def get_destination_mac(self):
        return self.destination_mac

    def set_destination_mac(self, destination_mac):
        self.destination_mac = destination_mac

    def get_sequence_number(self):
        return self.sequence_number

    def set_sequence_number(self, sequence_number):
        self.sequence_number = sequence_number

    def get_source_endpoint(self):
        return self.source_endpoint

    def set_source_endpoint(self, source_endpoint):
        self.source_endpoint = source_endpoint

    def get_destination_endpoint(self):
        return self.destination_endpoint

    def set_destination_endpoint(self, destination_endpoint):
        self.destination_endpoint = destination_endpoint

    def get_command_id(self):
        return self.command_id

    def set_command_id(self, command_id):
        self.command_id = command_id

    def get_mac_of_last_response(self):
        return self.mac_of_last_response

    def set_mac_of_last_response(self, mac_of_last_response):
        self.mac_of_last_response = mac_of_last_response