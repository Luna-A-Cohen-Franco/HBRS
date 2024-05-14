use crate::classes::{endpoint::endpoint_value::EndpointValue, enums::endpoint_value_type::EndpointValueType};

#[derive(Debug, Clone)]
pub struct StatusLight{
    endpoint_values: Vec<EndpointValue>,
}

impl StatusLight{
    pub fn new() -> Self{
        Self{
            endpoint_values: Vec::new(),
        }
    }

    pub fn get_endpoint_values(&self) -> Vec<EndpointValue>{
        return self.endpoint_values.iter().cloned().collect();
    }

    pub fn set_bytes(&mut self, data: &[u8], header_offset: usize){
        let num = data[header_offset];
        let b = data[header_offset + 1];
        let num2 = num & 1;
        
        self.find_endpoint_value(EndpointValueType::IsOn, num2.to_string());
        self.find_endpoint_value(EndpointValueType::Dim, b.to_string());
    }

    pub fn find_endpoint_value(&mut self, value_type: EndpointValueType, value: String){
        match self.endpoint_values.iter_mut().find(|v| *v.get_value_type_ref() == EndpointValueType::HVACFlags){
            Some(endpoint_value) => {
                *endpoint_value.get_value_mut() = value;
            },
            None => {
                let mut endpoint_value = EndpointValue::new_empty();
                *endpoint_value.get_value_type_mut() = value_type;
                *endpoint_value.get_value_mut() = value;
                self.endpoint_values.push(endpoint_value);
            }
        }
    }

    pub fn get_endpoint_values_ref(&self) -> &Vec<EndpointValue> {
        &self.endpoint_values
    }
    
    pub fn get_endpoint_values_mut(&mut self) -> &mut Vec<EndpointValue> {
        &mut self.endpoint_values
    }
}