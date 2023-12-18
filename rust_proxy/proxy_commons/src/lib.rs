use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct Message {
    pub data: Vec<u8>,
    pub origin: (IpAddr, u16),
    pub destination: (IpAddr, u16),
    pub mask_as_address: Option<(IpAddr, u16)>,
}
