use crate::classes::enums::endpoint_value_type::EndpointValueType;

#[derive(Clone)]
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

    pub fn get_value(&self) -> String{
        return self.value.clone();
    }
    pub fn get_value_type(&self) -> EndpointValueType{
        return self.value_type.clone();
    }
    pub fn set_value(&mut self, value: String){
        self.value = value;
    }
    pub fn set_value_type(&mut self, value_type: EndpointValueType){
        self.value_type = value_type;
    }
}