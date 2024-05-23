from typing import List
from lib.classes.endpoint.endpoint_value import EndpointValue
from lib.consts.endpoint_value_type import EndpointValueType


class StatusHVAC:
    def __init__(self):
        self.endpoint_values = []

    def get_endpoint_values(self):
        return self.endpoint_values[:]

    def set_bytes(self, data: List[int], header_offset: int):
        b = data[header_offset]
        b2 = data[header_offset + 1]
        b3 = data[header_offset + 2]
        value = int.from_bytes(data[header_offset + 3:header_offset + 5], byteorder='big', signed=False) / 100.0
        value2 = int.from_bytes(data[header_offset + 5:header_offset + 7], byteorder='big', signed=False) / 100.0
        num = int.from_bytes(data[header_offset + 7:header_offset + 9], byteorder='big', signed=False)
        num2 = int.from_bytes(data[header_offset + 9:header_offset + 11], byteorder='big', signed=False)

        self.find_endpoint_value(EndpointValueType.ThermostatMode, str(b))
        self.find_endpoint_value(EndpointValueType.ThermostatFanMode, str(b2))
        self.find_endpoint_value(EndpointValueType.MeasuredTemperatureC, str(value))
        self.find_endpoint_value(EndpointValueType.ThermostatDesiredTempC, str(value2))
        self.find_endpoint_value(EndpointValueType.HVACFlags, str(b3))
        self.find_endpoint_value(EndpointValueType.HVACTimerOnMinutes, str(num))
        self.find_endpoint_value(EndpointValueType.HVACTimerOffMinutes, str(num2))

    def find_endpoint_value(self, value_type, value):
        for endpoint_value in self.endpoint_values:
            if endpoint_value.get_value_type() == EndpointValueType.HVACFlags:
                endpoint_value.set_value(value)
                return

        endpoint_value = EndpointValue()
        endpoint_value.set_value_type(value_type)
        endpoint_value.set_value(value)
        self.endpoint_values.append(endpoint_value)
