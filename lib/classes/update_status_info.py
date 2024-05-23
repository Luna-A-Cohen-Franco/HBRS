from typing import List
from lib.classes.addresses.ipv4 import IPv4Addr
from lib.classes.endpoint.endpoint_value import EndpointValue


class UpdateStatusInfo:
    def __init__(self, is_ack: bool, endpoint_id: int, local_ip_address: IPv4Addr, endpoint_values: List[EndpointValue]):
        self.is_ack = is_ack
        self.endpoint_id = endpoint_id
        self.local_ip_address = local_ip_address
        self.endpoint_values = endpoint_values

    def is_ack(self):
        return self.is_ack

    def get_endpoint_id(self):
        return self.endpoint_id

    def get_local_ip_address(self):
        return self.local_ip_address

    def get_endpoint_values(self):
        return self.endpoint_values

    def set_is_ack(self, is_ack: bool):
        self.is_ack = is_ack

    def set_endpoint_id(self, endpoint_id: int):
        self.endpoint_id = endpoint_id
    
    def set_local_ip_address(self, local_ip_address: IPv4Addr):
        self.local_ip_address = local_ip_address
    
    def set_endpoint_values(self, endpoint_values: List[EndpointValue]):
        self.endpoint_values = endpoint_values
    