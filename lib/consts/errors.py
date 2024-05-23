from enum import Enum

class HbrsError(Enum):
    DataLengthExceedsLimit = "Data length exceeds string size"
    BadMacAddressLength = "Invalid MAC address size"
    BadMacAddressValues = "Invalid MAC address values"
    BadIpv4AddressLength = "Invalid IPv4 address size"
    InvalidScanDataLength = "Invalid scan data length"
    BadSSID = "Bad SSID"
    BadJoinEnumRes = "Bad join enum response data"