class HVACComm:
    def __init__(self):
        self.comm = 0

    def get_byte(self):
        return self.comm

    def set_byte(self, comm):
        self.comm = comm