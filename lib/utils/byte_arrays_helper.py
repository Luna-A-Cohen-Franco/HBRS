from typing import List
from lib.consts.errors import HbrsError


class ByteArraysHelper:
    @staticmethod
    def cp_arr_string_fill(data: str, strict_length: int) -> bytes:
        data2 = data.encode('utf-8')
        return ByteArraysHelper.cp_arr_bytes_fill(data2, strict_length)

    @staticmethod
    def cp_arr_bytes_fill(data: bytes, strict_length: int) -> bytes:
        array = bytearray(strict_length)
    
        array[:len(data)] = data[:strict_length]
    
        return array

    @staticmethod
    def combine_2v(first: List[int], second: List[int]) -> bytes:
        return bytes(first) + bytes(second)

    @staticmethod
    def combine_1v_1b(first: List[int], second: int) -> bytes:
        return bytes(first) + bytes([second])
    
    def combine_1b_1v(first: int, second: List[int]) -> bytes:
        return bytes([first]) + bytes(second)
     
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