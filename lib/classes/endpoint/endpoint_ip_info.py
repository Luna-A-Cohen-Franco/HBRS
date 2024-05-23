class EndpointIPInfo:
    def __init__(self, device_id: int, endpoint_id: int, device_mac: str, endpoint_address: str):
        self.device_id = device_id
        self.endpoint_id = endpoint_id
        self.device_mac = device_mac
        self.endpoint_address = endpoint_address
    def get_device_id(self):
        return self.device_id
    def get_endpoint_id(self):
        return self.endpoint_id
    def get_device_mac(self):
        return self.device_mac
    def get_endpoint_address(self):
        return self.endpoint_address
    
    def set_device_id(self, device_id):
        self.device_id = device_id
    def set_endpoint_id(self, endpoint_id):
        self.endpoint_id = endpoint_id
    def set_device_mac(self, device_mac):
        self.device_mac = device_mac
    def set_endpoint_address(self, endpoint_address):
        self.endpoint_address = endpoint_address