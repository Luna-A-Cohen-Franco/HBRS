from lib.utils.byte_arrays_helper import ByteArraysHelper


class Ping:
    def __init__(self):
        self.length = 1
        self.data = [65]

    def get_bytes(self):
        return ByteArraysHelper.combine_1b_1v(self.length, self.data)

    def set_bytes(self, data):
        self.length = 1
        self.data = data
