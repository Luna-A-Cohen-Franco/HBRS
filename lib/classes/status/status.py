from lib.classes.status.status_hvac import StatusHVAC
from lib.classes.status.status_light import StatusLight


class Status:
    def __init__(self):
        self.endpoint_values = []
        self.status_type = 0
        self.light = None
        self.hvac = None
        #self.curtain = None
        #self.ias_sensor = None
        #self.door_lock = None

    def update_status(self, status_type):
        self.status_type = status_type
        self.endpoint_values = self.match_endpoint_values()

    def match_endpoint_values(self):
        if self.status_type == 1:
            if self.light is None:
                self.light = StatusLight()
            return self.light.get_endpoint_values()
        elif self.status_type == 6:
            if self.hvac is None:
                self.hvac = StatusHVAC()
            return self.hvac.get_endpoint_values()
        return []
    def set_bytes(self, data, header_offset):
        self.status_type = data[header_offset]
        if self.status_type == 6:
            if self.hvac is None:
                self.hvac = StatusHVAC()
            self.hvac.set_bytes(data, header_offset + 1)
        elif self.status_type == 1:
            if self.light is None:
                self.light = StatusLight()
            self.light.set_bytes(data, header_offset + 1)
        return True

    def get_endpoint_values(self):
        return self.endpoint_values

    def get_status_type(self):
        return self.status_type

    def get_light(self):
        return self.light

    def set_light(self, light):
        self.light = light

    def get_hvac(self):
        return self.hvac

    def set_hvac(self, hvac):
        self.hvac = hvac

    """
    def get_curtain(self):
        return self.curtain

    def set_curtain(self, curtain):
        self.curtain = curtain

    def get_ias_sensor(self):
        return self.ias_sensor

    def set_ias_sensor(self, ias_sensor):
        self.ias_sensor = ias_sensor

    def get_door_lock(self):
        return self.door_lock

    def set_door_lock(self, door_lock):
        self.door_lock = door_lock
    """
    
