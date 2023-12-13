use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr};
use tracing_log::log::{debug, info};

use proxy_commons::Message;

#[no_mangle]
pub fn set_shared_logger(logger: proxy_commons::shared_logger::SharedLogger) {
    proxy_commons::shared_logger::setup_shared_logger(logger)
}

#[no_mangle]
pub fn parse_message(message: &mut Message) {
    info!("Parsing message.");
    let data = message.data.as_mut_slice();

    let maybe_dplay_signature = if data.len() >= 28 { &data[20..24] } else { &[] };

    if maybe_dplay_signature == [112, 108, 97, 121] {
        let mut size_and_token_combined_slice = [0_u8; 4];
        size_and_token_combined_slice.copy_from_slice(&data[0..4]);
        let size_and_token_combined_u32 = u32::from_le_bytes(size_and_token_combined_slice);
        let size_u20 = size_and_token_combined_u32 & 0x000fffff;
        let token_u12 = (size_and_token_combined_u32 & 0xfff00000) >> 20;
        let mut sock_addr_in_cursor = Cursor::new(&data[4..20]);
        let sock_addr_in_address_family = sock_addr_in_cursor.read_u16::<LittleEndian>().unwrap();
        let sock_addr_in_port = sock_addr_in_cursor.read_u16::<BigEndian>().unwrap();
        let sock_addr_in_ip_address = sock_addr_in_cursor.read_u32::<BigEndian>().unwrap();
        let mut version_and_command_cursor = Cursor::new(&data[24..28]);
        // TODO: parse command with enum containing all valid dplay commands
        let command = version_and_command_cursor
            .read_u16::<LittleEndian>()
            .unwrap();
        let version = version_and_command_cursor
            .read_u16::<LittleEndian>()
            .unwrap();
        debug!("DPLAY, size: {}, token: 0x{:x}, SockAddr: [AF: 0x{:x}, port: {}, ip_addr: 0x{:x}, signature: play, version: {:?}, command: {:?}]", size_u20, token_u12, sock_addr_in_address_family, sock_addr_in_port, sock_addr_in_ip_address, version, command);
        if let Some((address, _)) = message.mask_as_address {
            if address.is_ipv4() {
                if let IpAddr::V4(address) = address {
                    debug!("Injecting proxy IP");
                    data[8..12].copy_from_slice(&(address as Ipv4Addr).octets());
                }
            }
        }
    } else {
        debug!("Package data {:?}", data);
    }
}
