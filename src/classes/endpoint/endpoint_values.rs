use super::endpoint_value::EndpointValue;

pub struct EndpointValues{
    endpoint_id: u64,
    values: Vec<EndpointValue>,
}

impl EndpointValues{
    pub fn new(endpoint_id: u64, values: Vec<EndpointValue>) -> Self{
        Self{
            endpoint_id,
            values,
        }
    }
}