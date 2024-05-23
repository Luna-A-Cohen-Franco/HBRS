use crate::classes::enums::endpoint_value_type::EndpointValueType;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct EndpointValue{
    value_type: EndpointValueType,
    value: String,
}

impl EndpointValue{
    pub fn new(value_type: EndpointValueType, value: String) -> Self{
        Self{
            value_type,
            value,
        }
    }
    pub fn new_empty() -> Self{
        Self{
            value_type: EndpointValueType::Unknown,
            value: String::new(),
        }
    }

    pub fn get_value_type_ref(&self) -> &EndpointValueType {
        return &self.value_type;
    }

    pub fn get_value_type_mut(&mut self) -> &mut EndpointValueType {
        return &mut self.value_type;
    }

    pub fn get_value_ref(&self) -> &String {
        return &self.value;
    }

    pub fn get_value_mut(&mut self) -> &mut String {
        return &mut self.value;
    }
}