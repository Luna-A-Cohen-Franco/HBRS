from lib.classes.endpoint.endpoint_value import EndpointValue
from lib.consts.endpoint_value_type import EndpointValueType


class StatusLight:
    def __init__(self):
        self.endpoint_values = []

    def get_endpoint_values(self):
        return self.endpoint_values

    def set_bytes(self, data, header_offset):
        num = data[header_offset]
        b = data[header_offset + 1]
        num2 = num & 1

        self.find_endpoint_value(EndpointValueType.IsOn, str(num2))
        self.find_endpoint_value(EndpointValueType.Dim, str(b))

    def find_endpoint_value(self, value_type, value):
        for endpoint_value in self.endpoint_values:
            if endpoint_value.get_value_type() == value_type:
                endpoint_value.set_value(value)
                return

        endpoint_value = EndpointValue()
        endpoint_value.set_value_type(value_type)
        endpoint_value.set_value(value)
        self.endpoint_values.append(endpoint_value)