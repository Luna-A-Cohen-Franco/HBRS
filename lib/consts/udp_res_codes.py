from enum import Enum

class UDPResponse(Enum):
    _None = 0
    JoinEnumerateResponse = 1
    JoinScanResponseStarted = 2
    JoinSwitchCloudResponse = 3
    JoinJoinResponse = 4
