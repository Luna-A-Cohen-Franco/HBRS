use super::{addresses::ipv4::IPv4Addr, endpoint::endpoint_value::EndpointValue};

#[derive(Debug, Clone)]
pub struct UpdateStatusInfo {
    pub is_ack: bool,
    pub endpoint_id: u64,
    pub local_ip_address: IPv4Addr,
    pub endpoint_values: Vec<EndpointValue>,
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

    pub fn get_is_ack(&self) -> bool{
        self.is_ack
    }

    pub fn get_endpoint_id(&self) -> u64{
        self.endpoint_id
    }

    pub fn get_local_ip_address(&self) -> IPv4Addr{
        self.local_ip_address.clone()
    }

    pub fn get_endpoint_values(&self) -> Vec<EndpointValue>{
        self.endpoint_values.clone()
    }

    pub fn set_is_ack(&mut self, is_ack: bool){
        self.is_ack = is_ack;
    }

    pub fn set_endpoint_id(&mut self, endpoint_id: u64){
        self.endpoint_id = endpoint_id;
    }

    pub fn set_local_ip_address(&mut self, local_ip_address: IPv4Addr){
        self.local_ip_address = local_ip_address;
    }

    pub fn set_endpoint_values(&mut self, endpoint_values: Vec<EndpointValue>){
        self.endpoint_values = endpoint_values;
    }
}