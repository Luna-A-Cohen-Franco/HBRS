from lib.classes.join.scan_res_item import ScanResItem
from lib.consts.errors import HbrsError


class ScanRes:
    def __init__(self):
        self.wifis = []

    def add_new_item(self, data, header_offset):
        new_item = ScanResItem(data, header_offset)
        self.wifis.append(new_item)
        """
        if new_item.get_ssid() == b'\x00' * 32 or new_item.get_ssid().decode('utf-8').strip() == '' or new_item.get_rssi() <= 0:
            raise HbrsError.BadSSID

        existing_item = next((i for i in self.wifis if i.get_ssid().decode('utf-8').strip().lower() == new_item.get_ssid().decode('utf-8').strip().lower()), None)
        if existing_item:
            if existing_item.get_rssi() < new_item.get_rssi():
                existing_item.set_rssi(new_item.get_rssi())
        else:
            self.wifis.append(new_item)
        return True
        """
        
        
    def get_bytes(self):
        bytes = b''
        for wifi in self.wifis:
            bytes += wifi.get_bytes()
        return bytes

    def get_wifi(self, ssid):
        return next((i for i in self.wifis if i.get_ssid().decode('utf-8').strip().lower() == ssid.strip().lower()), None)

    def get_wifi_by_index(self, index):
        return self.wifis[index]

    def get_wifis(self):
        return self.wifis