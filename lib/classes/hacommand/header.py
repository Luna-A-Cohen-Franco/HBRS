from lib.classes.addresses.mac import MacAddress
from lib.consts.other import Other


class Header:
    def __init__(self):
        self.protocol_version = 0
        self.source_mac = MacAddress()
        self.destination_mac = MacAddress()
        self.sequence_number = 0
        self.source_endpoint = 0
        self.destination_endpoint = 0
        self.command_id = 0
        self.mac_of_last_response = MacAddress()

    def get_bytes(self):
        bytes = []
        for _ in range(Other.HeaderOffset):
            bytes.append(0)

        index = 0

        bytes[index] = self.protocol_version
        index += 1

        for byte in self.source_mac.get_bytes_ref():
            bytes[index] = byte
            index += 1

        index = 7

        for byte in self.destination_mac.get_bytes_ref():
            bytes[index] = byte
            index += 1

        index = 13
        bytes[index] = self.sequence_number
        index += 1
        bytes[index] = self.source_endpoint
        index += 1
        bytes[index] = self.destination_endpoint
        index += 1
        bytes[index] = self.command_id

        return bytes

    def set_bytes(self, data):
        index = 0

        self.protocol_version = data[index]
        index += 1

        self.mac_of_last_response = self.source_mac.clone()
        source_mac_bytes = data[index:index + 6]
        self.source_mac = MacAddress.new(source_mac_bytes)

        index = 7
        destination_mac_bytes = data[index:index + 6]
        self.destination_mac = MacAddress.new(destination_mac_bytes)

        index = 13
        self.sequence_number = data[index]
        index += 1
        self.source_endpoint = data[index]
        index += 1
        self.destination_endpoint = data[index]
        index += 1
        self.command_id = data[index]

    def clear_bytes(self):
        self.protocol_version = 0
        self.source_mac = MacAddress.new_empty()
        self.destination_mac = MacAddress.new_empty()
        self.sequence_number = 0
        self.source_endpoint = 0
        self.destination_endpoint = 0
        self.command_id = 0
        self.mac_of_last_response = MacAddress.new_empty()

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