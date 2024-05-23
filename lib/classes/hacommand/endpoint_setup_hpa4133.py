class EndpointSetupHPA4133:
    def __init__(self):
        self.start_type = 1

    def get_bytes(self):
        second = [0] * 8
        return second + [self.start_type]

    def get_start_type(self):
        return self.start_type

    def set_start_type(self, start_type):
        self.start_type = start_type