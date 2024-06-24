class MacAddress:
    def __init__(self, bytes: list[int]):
        self.bytes = bytes

    def get_bytes(self):
        return self.bytes
    
    def set_bytes(self, bytes: list[int]):
        self.bytes = bytes

    @staticmethod
    def from_string(addr: str):
        if len(addr) != 12:
            raise ValueError("Bad MAC address length")

        for c in addr:
            if not c.isdigit() and c not in "abcdef":
                raise ValueError("Bad MAC address values")

        bytes = []
        for i in range(0, len(addr), 2):
            byte = int(addr[i:i+2], 16)
            bytes.append(byte)

        return MacAddress(bytes)

    def display(self):
        print(self.to_string())

    def to_string(self):
        result = []
        for byte in self.bytes:
            result.append(f"{byte:02X}:")
        return ''.join(result)[:-1]
            
    @staticmethod
    def new_empty():
        return MacAddress([])

    @staticmethod
    def new(bytes: list[int]):
        if len(bytes) != 6:
            raise ValueError("Bad MAC address length")

        return MacAddress(bytes)

