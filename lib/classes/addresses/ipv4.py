from typing import List


class IPv4Addr:
    def __init__(self, bytes: List[int]):
        self.bytes = bytes

    def get_bytes(self):
        return self.bytes

    def set_bytes(self, bytes: List[int]):
        self.bytes = bytes