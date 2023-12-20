use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use bytes::Buf;
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr};

use proxy_commons::Message;

#[no_mangle]
pub fn parse_message(message: &mut Message) {
    // println!("Parsing message.");
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
        let command_string = match_dplay_command(command);
        match command_string.as_str() {
            "Ping" => (),
            "PingReply" => (),
            _ => {
                println!("[DPLAY] size: {}, token: 0x{:x}, SockAddr: [AF: 0x{:x}, port: {}, ip_addr: 0x{:x}, signature: play, version: {:?}, command: {}]", 
                    size_u20,
                    token_u12,
                    sock_addr_in_address_family,
                    sock_addr_in_port,
                    sock_addr_in_ip_address,
                    version,
                    command_string
                );
            }
        }

        if let Some((address, _)) = message.mask_as_address {
            if address.is_ipv4() {
                if let IpAddr::V4(address) = address {
                    println!("Injecting proxy IP");
                    data[8..12].copy_from_slice(&(address as Ipv4Addr).octets());
                }
            }
        }
    } else {
        let packet_length = data.len();
        let mut cursor = Cursor::new(&data);
        let id1 = cursor.read_u32::<LittleEndian>().unwrap();
        let id2 = cursor.read_u32::<LittleEndian>().unwrap();
        let data = &data[8..];
        let mut cursor = Cursor::new(&data);
        let action_token = cursor.read_u32::<LittleEndian>().unwrap();
        // Size seems to be excluding the IDs
        let action_length = cursor.read_u32::<LittleEndian>().unwrap();
        let rest_of_data = &data[cursor.position() as usize..];
        match action_token {
            0x07D1 => print_game_token_generic(
                id1,
                id2,
                "Defocus",
                action_length,
                packet_length,
                hex::encode(rest_of_data).as_str(),
            ),
            0x07D2 => print_game_token_generic(
                id1,
                id2,
                "Focus",
                action_length,
                packet_length,
                hex::encode(rest_of_data).as_str(),
            ),
            0x0836 => print_game_token_generic(
                id1,
                id2,
                "Move/ShipAction",
                action_length,
                packet_length,
                hex::encode(rest_of_data).as_str(),
            ),
            0x084F => print_game_token_generic(
                id1,
                id2,
                "TogglePause+Unknown",
                action_length,
                packet_length,
                hex::encode(rest_of_data).as_str(),
            ),
            0x0853 => {
                cursor.advance(10);
                let to_player = cursor.read_u16::<LittleEndian>().unwrap();
                let from_player = cursor.read_u16::<LittleEndian>().unwrap();
                let chat_message_raw = &rest_of_data[16..76];
                let chat_message_range_end = chat_message_raw
                    .iter()
                    .position(|&c| c == b'\0')
                    .unwrap_or(chat_message_raw.len());
                let chat_message =
                    ::std::str::from_utf8(&chat_message_raw[0..chat_message_range_end])
                        .unwrap_or(">> Couldn't parse message. <<");
                println!(
                    "[X642::Chat::RAW] {}\n[X642::Chat] {} to {}: {}",
                    hex::encode(data),
                    player_id_to_string(from_player),
                    player_id_to_string(to_player),
                    chat_message
                )
            }
            _ => print_game_token_generic(
                id1,
                id2,
                format!("0x{:04x}", action_token).as_str(),
                action_length,
                packet_length,
                hex::encode(rest_of_data).as_str(),
            ),
        }
    }
}

fn print_game_token_generic(
    id1: u32,
    id2: u32,
    action_token: &str,
    action_length: u32,
    packet_length: usize,
    rest_of_data_hex: &str,
) {
    println!(
        "[X642] 0x{:08x} -> 0x{:08x} | TKN: {}, LEN: {}:{} | {}",
        id1, id2, action_token, action_length, packet_length, rest_of_data_hex
    )
}

fn player_id_to_string(player_id: u16) -> &'static str {
    match player_id {
        0 => "PLAYER_ONE",
        1 => "PLAYER_TWO",
        2 => "PLAYER_THREE",
        3 => "PLAYER_FOUR",
        0xff => "ALL_PLAYERS",
        _ => "UNKNOWN",
    }
}

fn match_dplay_command(command: u16) -> String {
    match command {
        0x0001 => String::from("ENUMSESSIONSREPLY"),
        0x0002 => String::from("ENUMSESSIONS"),
        0x0003 => String::from("ENUMPLAYERSREPLY"),
        0x0004 => String::from("ENUMPLAYER"),
        0x0005 => String::from("REQUESTPLAYERID"),
        0x0006 => String::from("REQUESTGROUPID"),
        0x0007 => String::from("REQUESTPLAYERREPLY"),
        0x0008 => String::from("CREATEPLAYER"),
        0x0009 => String::from("CREATEGROUP"),
        0x000A => String::from("PLAYERMESSAGE"),
        0x000B => String::from("DELETEPLAYER"),
        0x000C => String::from("DELETEGROUP"),
        0x000D => String::from("ADDPLAYERTOGROUP"),
        0x000E => String::from("DELETEPLAYERFROMGROUP"),
        0x000F => String::from("PLAYERDATACHANGED"),
        0x0010 => String::from("PLAYERNAMECHANGED"),
        0x0011 => String::from("GROUPDATACHANGED"),
        0x0012 => String::from("GROUPNAMECHANGED"),
        0x0013 => String::from("ADDFORWARDREQUEST"),
        // 0x0014 not assigned
        0x0015 => String::from("PACKET"),
        0x0016 => String::from("Ping"),
        0x0017 => String::from("PingReply"),
        0x0018 => String::from("YOUAREDEAD"),
        0x0019 => String::from("PLAYERWRAPPER"),
        0x001A => String::from("SESSIONDESCCHANGED"),
        // 0x001B not assigned
        0x001C => String::from("CHALLENGE"),
        0x001D => String::from("ACCESSGRANTED"),
        0x001E => String::from("LOGONDENIED"),
        0x001F => String::from("AUTHERROR"),
        0x0020 => String::from("NEGOTIATE"),
        0x0021 => String::from("CHALLENGERESPONSE"),
        0x0022 => String::from("SIGNED"),
        // 0x0023 not assigned
        0x0024 => String::from("ADDFORWARDREPLY"),
        0x0025 => String::from("ASK4MULTICAST"),
        0x0026 => String::from("ASK4MULTICASTGUARANTEED"),
        0x0027 => String::from("ADDSHORTCUTTOGROUP"),
        0x0028 => String::from("DELETEGROUPFROMGROUP"),
        0x0029 => String::from("SUPERENUMPLAYERSREPLY"),
        // 0x002A not assigned
        0x002B => String::from("KEYEXCHANGE"),
        0x002C => String::from("KEYEXCHANGEREPLY"),
        0x002D => String::from("CHAT"),
        0x002E => String::from("ADDFORWARD"),
        0x002F => String::from("ADDFORWARDACK"),
        0x0030 => String::from("PACKET2_DATA"),
        0x0031 => String::from("PACKET2_ACK"),
        // 0x0032 - 0x0034 not assigned
        0x0035 => String::from("IAMNAMESERVER"),
        0x0036 => String::from("VOICE"),
        0x0037 => String::from("MULTICASTDELIVERY"),
        0x0038 => String::from("CREATEPLAYERVERIFY"),
        _ => format!("0x{:04x}", command),
    }
}
