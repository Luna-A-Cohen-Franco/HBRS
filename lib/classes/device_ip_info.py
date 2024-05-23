class DeviceIPInfo:
    def __init__(self, device_id: int, device_mac: str, local_ip_address: str):
        self.device_id = device_id
        self.device_mac = device_mac
        self.local_ip_address = local_ip_address

    def get_device_id(self):
        return self.device_id
    def get_device_mac(self):
        return self.device_mac
    def get_local_ip_address(self):
        return self.local_ip_address
    
    def set_device_id(self, device_id):
        self.device_id = device_id
    def set_device_mac(self, device_mac):
        self.device_mac = device_mac
    def set_local_ip_address(self, local_ip_address):
        self.local_ip_address = local_ip_address
    