from lib.utils.byte_arrays_helper import ByteArraysHelper


class CloudNotifIP:
    def __init__(self, address: list):
        self.address = ByteArraysHelper.cp_arr_bytes_fill(address)

    def get_bytes(self) -> list:
        return self.address

    def set_bytes(self, address: list) -> None:
        self.address = ByteArraysHelper.cp_arr_bytes_fill(address)

    
