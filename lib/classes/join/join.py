from lib.classes.join.enum_res import EnumRes
from lib.classes.join.scan_res import ScanRes


class Join:
    def __init__(self):
        self.sub_command = 0
        self.join_req = None
        self.enum_res = None
        self.scan_res = None
        self.cloud_notif_ip = None

    def get_bytes(self):
        data = [self.sub_command]

        if self.sub_command == 8 and self.enum_res is not None:
            return data + self.cloud_notif_ip.get_bytes()

        if self.sub_command == 3 and self.join_req is not None:
            return bytes(data) + self.join_req.get_bytes()

        return data

    def set_bytes(self, data, header_offset, sub_comm_res):        
        if sub_comm_res == 2:
            if self.enum_res is None:
                self.enum_res = EnumRes()
                self.enum_res.set_bytes(data, header_offset + 1)
                return True

            self.enum_res.set_bytes(data, header_offset + 1)
            return True

        if sub_comm_res == 6:
            self.scan_res = ScanRes()
            
            self.scan_res.add_new_item(data, header_offset)
            return True

        return False

    def get_sub_command(self):
        return self.sub_command
    
    def set_sub_command(self, sub_command):
        self.sub_command = sub_command

    def get_join_req(self):
        return self.join_req

    def set_join_req(self, join_req):
        self.join_req = join_req

    def get_enum_res(self):
        return self.enum_res

    def set_enum_res(self, enum_res):
        self.enum_res = enum_res

    def get_scan_res(self):
        return self.scan_res

    def set_scan_res(self, scan_res):
        self.scan_res = scan_res

    def get_cloud_notif_ip(self):
        return self.cloud_notif_ip

    def set_cloud_notif_ip(self, cloud_notif_ip):
        self.cloud_notif_ip = cloud_notif_ip