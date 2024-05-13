use super::{addresses::ipv4::IPv4Addr, endpoint::endpoint_value::EndpointValue};

pub struct UpdateStatusInfo {
    pub is_ack: bool,
    pub endpoint_id: u64,
    pub local_ip_address: IPv4Addr,
    pub endpoint_values: Vec<EndpointValue>,
}