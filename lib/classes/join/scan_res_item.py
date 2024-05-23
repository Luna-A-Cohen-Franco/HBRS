class ScanResItem:
    def __init__(self, data, header_offset):
        self.ssid = data[header_offset:header_offset+ 32]
        self.security_type = data[header_offset + 33]
        self.encryption_type = data[header_offset + 34]
        self.rssi = data[header_offset + 35]

    def get_ssid_as_str(self):
        return self.ssid.decode('utf-8')

    def get_bytes(self):
        return self.ssid + bytes([self.security_type, self.encryption_type, self.rssi])

    def get_ssid(self):
        return self.ssid

    def set_ssid(self, ssid):
        self.ssid = ssid

    def get_security_type(self):
        return self.security_type

    def set_security_type(self, security_type):
        self.security_type = security_type

    def get_encryption_type(self):
        return self.encryption_type

    def set_encryption_type(self, encryption_type):
        self.encryption_type = encryption_type

    def get_rssi(self):
        return self.rssi

    def set_rssi(self, rssi):
        self.rssi = rssi