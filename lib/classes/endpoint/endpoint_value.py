from lib.consts.endpoint_value_type import EndpointValueType


class EndpointValue:
    def __init__(self, value_type: EndpointValueType, value: str):
        self.value_type = value_type
        self.value = value

    def get_value_type(self):
        return self.value_type

    def get_value(self):
        return self.value

    def set_value_type(self, value_type: EndpointValueType):
        self.value_type = value_type
    
    def set_value(self, value: str):
        self.value = value