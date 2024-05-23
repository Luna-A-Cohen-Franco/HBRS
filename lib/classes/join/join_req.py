from lib.consts.other import Other
from lib.utils.byte_arrays_helper import ByteArraysHelper


class JoinReq:
    def __init__(self, ssid: list[int], security_type: int, encryption_type: int, key: list[int]):
        self.ssid = ssid
        self.security_type = security_type
        self.encryption_type = encryption_type
        self.key = key

    @staticmethod
    def new(ssid: list[int], security_type: int, encryption_type: int, key: list[int]):
        key = ByteArraysHelper.cp_arr_bytes_fill(key, Other.KeyLength)
        return JoinReq(ssid, security_type, encryption_type, key)

    def get_bytes(self) -> list[int]:
        first = ByteArraysHelper.combine_1v_1b(self.ssid, self.security_type)
        second = ByteArraysHelper.combine_1v_1b(self.key, self.encryption_type)
        return ByteArraysHelper.combine_2v(first, second)
    
    def set_key(self, key: list[int]) -> None:
        self.key = ByteArraysHelper.cp_arr_bytes_fill(key, Other.KeyLength)

    def get_ssid(self):
        return self.ssid

    def set_ssid(self, ssid: list[int]):
        self.ssid = ssid

    def get_security_type(self):
        return self.security_type

    def set_security_type(self, security_type: int):
        self.security_type = security_type

    def get_encryption_type(self):
        return self.encryption_type

    def set_encryption_type(self, encryption_type: int):
        self.encryption_type = encryption_type

    def get_key(self):
        return self.key

    def set_key(self, key: list[int]):
        self.key = key
