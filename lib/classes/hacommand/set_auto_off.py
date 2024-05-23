from lib.utils.byte_arrays_helper import ByteArraysHelper


class SetAutoOff:
    def __init__(self):
        self.off_timeout = 0

    def get_bytes(self):
        b1, b2 = ByteArraysHelper.word_to_port16(self.off_timeout)
        return ByteArraysHelper.combine_2b(b1, b2)

    def set_bytes(self, data):
        self.off_timeout = ByteArraysHelper.port16_to_word(data)
        