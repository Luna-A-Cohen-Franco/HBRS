class SetMode:
    def __init__(self):
        self.mode = 0
        self.fan_mode = 0
        self.flags = 0
        self.desired_temp_b0 = 0
        self.desired_temp_b1 = 0

    def get_bytes(self):
        return [self.mode, self.fan_mode, self.flags, self.desired_temp_b0, self.desired_temp_b1]

    def get_mode(self):
        return self.mode

    def set_mode(self, mode):
        self.mode = mode

    def get_fan_mode(self):
        return self.fan_mode

    def set_fan_mode(self, fan_mode):
        self.fan_mode = fan_mode

    def get_flags(self):
        return self.flags

    def set_flags(self, flags):
        self.flags = flags

    def get_desired_temp_b0(self):
        return self.desired_temp_b0

    def set_desired_temp_b0(self, desired_temp_b0):
        self.desired_temp_b0 = desired_temp_b0

    def get_desired_temp_b1(self):
        return self.desired_temp_b1

    def set_desired_temp_b1(self, desired_temp_b1):
        self.desired_temp_b1 = desired_temp_b1