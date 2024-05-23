from enum import Enum

class ResponseItem(Enum):
    SSIDLength = 32
    SecurityTypePos = 33
    EncryptionTypePos = 34
    RSSIPos = 35
    DataSize = 54
    MinRSSI = 20
