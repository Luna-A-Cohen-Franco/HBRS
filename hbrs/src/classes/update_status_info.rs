use super::{addresses::ipv4::IPv4Addr, endpoint::endpoint_value::EndpointValue};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct UpdateStatusInfo {
    is_ack: bool,
    endpoint_id: u64,
    local_ip_address: IPv4Addr,
    endpoint_values: Vec<EndpointValue>,
}

impl UpdateStatusInfo{
    pub fn new(is_ack: bool, endpoint_id: u64, local_ip_address: IPv4Addr, endpoint_values: Vec<EndpointValue>) -> Self{
        Self{
            is_ack,
            endpoint_id,
            local_ip_address,
            endpoint_values,
        }
    }

    pub fn get_is_ack_ref(&self) -> &bool {
        &self.is_ack
    }
    
    pub fn get_is_ack_mut(&mut self) -> &mut bool {
        &mut self.is_ack
    }
    
    pub fn get_endpoint_id_ref(&self) -> &u64 {
        &self.endpoint_id
    }
    
    pub fn get_endpoint_id_mut(&mut self) -> &mut u64 {
        &mut self.endpoint_id
    }
    
    pub fn get_local_ip_address_ref(&self) -> &IPv4Addr {
        &self.local_ip_address
    }
    
    pub fn get_local_ip_address_mut(&mut self) -> &mut IPv4Addr {
        &mut self.local_ip_address
    }
    
    pub fn get_endpoint_values_ref(&self) -> &Vec<EndpointValue> {
        &self.endpoint_values
    }
    
    pub fn get_endpoint_values_mut(&mut self) -> &mut Vec<EndpointValue> {
        &mut self.endpoint_values
    }
}