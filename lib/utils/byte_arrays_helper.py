from lib.consts.errors import HbrsError


class ByteArraysHelper:
    @staticmethod
    def cp_arr_string_fill(data: str, strict_length: int) -> bytes:
        data2 = data.encode('utf-8')
        return ByteArraysHelper.cp_arr_bytes_fill(data2, strict_length)

    @staticmethod
    def cp_arr_bytes_fill(data: bytes, strict_length: int) -> bytes:
        if len(data) < strict_length:
            raise HbrsError.DataLengthExceedsLimit

        return data.ljust(strict_length, b'\0')

    @staticmethod
    def combine_2v(first: bytes, second: bytes) -> bytes:
        return first + second

    @staticmethod
    def combine_1v_1b(first: bytes, second: int) -> bytes:
        return first + bytes([second])

    @staticmethod
    def combine_2b(first: int, second: int) -> bytes:
        return bytes([first, second])

    @staticmethod
    def port16_to_word(b0: int, b1: int) -> int:
        return (b1 << 8) + b0

    @staticmethod
    def word_to_port16(value: int) -> tuple:
        return (value & 0xFF, value >> 8)

    @staticmethod
    def short_to_port32(value: int) -> bytes:
        return value.to_bytes(4, byteorder='little')

    @staticmethod
    def port32_to_dword(b0: int, b1: int, b2: int, b3: int) -> int:
        return (b3 << 24) + (b2 << 16) + (b1 << 8) + b0

    @staticmethod
    def int16_to_vectorby(value: int) -> bytes:
        return value.to_bytes(2, byteorder='little')