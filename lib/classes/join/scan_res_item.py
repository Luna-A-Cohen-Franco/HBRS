class ScanResItem:
    def __init__(self, data, header_offset):
        self.ssid = bytearray(33)
        for i in range(33):
            self.ssid[i] = 0

        self.ssid[:32] = data[header_offset:header_offset+32]
        self.ssid = [byte for byte in self.ssid]
        self.security_type = data[header_offset + 33]
        self.encryption_type = data[header_offset + 34]
        self.rssi = data[header_offset + 35]

    def display(self):
        print(self.get_ssid_as_str())
        print(self.security_type)
        print(self.encryption_type)
        print(self.rssi)

    def get_ssid_as_str(self):
        return ''.join(chr(byte) for byte in self.ssid)

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