class EnumRes:
    def __init__(self, data: bytearray, header_offset: int):
        num = len(data) - header_offset
        app_ver_bytes = bytearray()
        firmware_ver = ""
        device_model = ""

        app_ver_bytes = data[header_offset:header_offset + num]
        app_ver = app_ver_bytes.decode("utf-8")

        app_ver_split = app_ver.split(",")

        if len(app_ver_split) != 0:
            device_model = app_ver_split[0]

        if len(app_ver_split) > 1:
            firmware_ver = app_ver_split[1]

        self.app_ver = app_ver
        self.device_model = device_model
        self.firmware_ver = firmware_ver

    def set_bytes(self, data: bytearray, header_offset: int):
        num = len(data) - header_offset
        app_ver_bytes = bytearray()

        app_ver_bytes = data[header_offset:header_offset + num]
        app_ver = app_ver_bytes.decode("utf-8")

        app_ver_split = app_ver.split(",")

        if len(app_ver_split) != 0:
            self.device_model = app_ver_split[0]

        if len(app_ver_split) > 1:
            self.firmware_ver = app_ver_split[1]

        return True

    def get_bytes(self):
        return self.app_ver.encode("utf-8")

    def get_app_ver(self):
        return self.app_ver

    def set_app_ver(self, app_ver):
        self.app_ver = app_ver

    def get_device_model(self):
        return self.device_model

    def set_device_model(self, device_model):
        self.device_model = device_model

    def get_firmware_ver(self):
        return self.firmware_ver

    def set_firmware_ver(self, firmware_ver):
        self.firmware_ver = firmware_ver