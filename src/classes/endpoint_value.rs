use super::enums::endpoint_value_type::EndpointValueType;

pub struct EndpointValue{
    value_type: EndpointValueType,
    value: String,
}

impl EndpointValue{
    pub fn new(value_type: EndpointValueType, value: String) -> EndpointValue{
        EndpointValue{
            value_type,
            value,
        }
    }
}