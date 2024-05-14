use crate::{classes::{endpoint::endpoint_value::EndpointValue, enums::endpoint_value_type::EndpointValueType}, utils::byte_arrays_helper::ByteArraysHelper};

#[derive(Debug, Clone)]
pub struct StatusHVAC{
    endpoint_values: Vec<EndpointValue>,
}

impl StatusHVAC{
    pub fn new() -> Self{
        Self{
            endpoint_values: Vec::new(),
        }
    }

    pub fn get_endpoint_values(&self) -> Vec<EndpointValue>{
        return self.endpoint_values.iter().cloned().collect();
    }

    pub fn set_bytes(&mut self, data: &[u8], header_offset: usize){
        let b = data[header_offset];
        let b2 = data[header_offset + 1];
        let b3 = data[header_offset + 2];
        let value = f64::from(ByteArraysHelper::port16_to_word(data[header_offset + 3], data[header_offset + 4])) / 100.0;
        let value2 = f64::from(ByteArraysHelper::port16_to_word(data[header_offset + 5], data[header_offset + 6])) / 100.0;
        let num = u16::from(ByteArraysHelper::port16_to_word(data[header_offset + 7], data[header_offset + 8]));
        let num2 = u16::from(ByteArraysHelper::port16_to_word(data[header_offset + 9], data[header_offset + 10]));

        self.find_endpoint_value(EndpointValueType::ThermostatMode, b.to_string());
        self.find_endpoint_value(EndpointValueType::ThermostatFanMode, b2.to_string());
        self.find_endpoint_value(EndpointValueType::MeasuredTemperatureC, value.to_string());
        self.find_endpoint_value(EndpointValueType::ThermostatDesiredTempC, value2.to_string());
        self.find_endpoint_value(EndpointValueType::HVACFlags, b3.to_string());
        self.find_endpoint_value(EndpointValueType::HVACTimerOnMinutes, num.to_string());
        self.find_endpoint_value(EndpointValueType::HVACTimerOffMinutes, num2.to_string());

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