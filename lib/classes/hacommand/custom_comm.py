class CustomComm:
    def __init__(self):
        self.command = 0
        self.data = []

    def set_bytes(self, data, header_offset):
        self.command = data[header_offset]
        self.data = []

    def get_bytes(self):
        if len(self.data) == 0:
            return [self.command]

        return self.data + [self.command]

    def get_command(self):
        return self.command
    
    def set_command(self, command):
        self.command = command

    def get_data(self):
        return self.data
    
    def set_data(self, data):
        self.data = data