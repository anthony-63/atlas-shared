use core::str;

pub const ATLAS_PACKET_SIZE: usize = 4096;

#[derive(Debug)]
pub enum AtlasPacket {
    ConnectionRequest,
    ConnectionResponse(String), // welcome_message
    LoginRequest(String, String, String), // username, password, game_version
    LoginResponse(u16, String), // code, temp_token
    RegisterRequest(String, String), // username, password
    RegisterResponse(u16), // code
    JoinChatroomRequest(String, String), // chat, token
    JoinChatroomResponse(u16), // code
    SendMessageRequest(String, String), // message, token
    SendMessageResponse(u16), // code
    RecvMessage(String, String), // username, message
}

#[repr(u16)]
pub enum AtlasResponseCodes {
    Success = 0,
    UsernameExists,
    UserDoesntExist,
    IncorrectPassword,
    IncorrectToken,
    ChatroomDoesntExist,
}

#[repr(u8)]
pub enum AtlasPacketTypes {
    ConnectionRequest = 0,
    ConnectionResponse,
    LoginRequest,
    LoginResponse,
    RegisterRequest,
    RegisterResponse,
    JoinChatroomRequest,
    JoinChatroomResponse,
    SendMessageRequest,
    SendMessageResponse,
    RecvMessage,
}

impl AtlasPacket {
    pub fn serialize(&self) -> Option<[u8; ATLAS_PACKET_SIZE]> {
        match self {
            AtlasPacket::ConnectionRequest =>
                AtlasPacketWriter::vec_to_raw_packet(vec![
                    AtlasPacketTypes::ConnectionRequest as u8]),

            AtlasPacket::ConnectionResponse(welcome_message) =>
                AtlasPacketWriter::vec_to_raw_packet(vec![
                    vec![AtlasPacketTypes::ConnectionResponse as u8],
                    Self::make_string(welcome_message)].concat()),

            AtlasPacket::LoginRequest(username, password, game_version) =>
                AtlasPacketWriter::vec_to_raw_packet(vec![
                    vec![AtlasPacketTypes::LoginRequest as u8],
                    Self::make_string(username),
                    Self::make_string(password),
                    Self::make_string(game_version)].concat()),
            
            AtlasPacket::LoginResponse(code, temp_token) =>
                AtlasPacketWriter::vec_to_raw_packet(vec![
                    vec![AtlasPacketTypes::LoginResponse as u8],
                    Self::make_u16(code),
                    Self::make_string(temp_token)].concat()),

            AtlasPacket::RegisterRequest(username, password) =>
                AtlasPacketWriter::vec_to_raw_packet(vec![
                    vec![AtlasPacketTypes::RegisterRequest as u8],
                    Self::make_string(username),
                    Self::make_string(password)].concat()),

            AtlasPacket::RegisterResponse(code) => 
                AtlasPacketWriter::vec_to_raw_packet(vec![
                    vec![AtlasPacketTypes::RegisterResponse as u8],
                    Self::make_u16(code),
                ].concat()),
            
            AtlasPacket::JoinChatroomRequest(room, token) =>
                AtlasPacketWriter::vec_to_raw_packet(vec![
                    vec![AtlasPacketTypes::JoinChatroomRequest as u8],
                    Self::make_string(room),
                    Self::make_string(token),
                ].concat()),
            
            AtlasPacket::JoinChatroomResponse(code) => 
                AtlasPacketWriter::vec_to_raw_packet(vec![
                    vec![AtlasPacketTypes::JoinChatroomResponse as u8],
                    Self::make_u16(code),
                ].concat()),

            AtlasPacket::SendMessageRequest(message, token) =>
                AtlasPacketWriter::vec_to_raw_packet(vec![
                    vec![AtlasPacketTypes::SendMessageRequest as u8],
                    Self::make_string(message),
                    Self::make_string(token),
                ].concat()),

            AtlasPacket::SendMessageResponse(code) => 
                AtlasPacketWriter::vec_to_raw_packet(vec![
                    vec![AtlasPacketTypes::SendMessageResponse as u8],
                    Self::make_u16(code),
                ].concat()),

            AtlasPacket::RecvMessage(username, message) =>
                AtlasPacketWriter::vec_to_raw_packet(vec![
                    vec![AtlasPacketTypes::RecvMessage as u8],
                    Self::make_string(username),
                    Self::make_string(message),
                ].concat()),
        }
    }

    fn make_string(s: &str) -> Vec<u8> {
        return vec![&s.len().to_be_bytes(), s.as_bytes()].concat();
    }
    
    fn make_bool(b: &bool) -> Vec<u8> {
        return vec![if *b == false { 0 } else { 1 }]
    }

    fn make_u16(n: &u16) -> Vec<u8> {
        return n.to_be_bytes().to_vec();
    }
}

pub struct AtlasPacketWriter;

impl AtlasPacketWriter {
    pub fn vec_to_raw_packet(vec: Vec<u8>) -> Option<[u8; ATLAS_PACKET_SIZE]> {
        if vec.len() >= ATLAS_PACKET_SIZE {
            return None;
        }

        let mut data: [u8; ATLAS_PACKET_SIZE] = [0; ATLAS_PACKET_SIZE];

        for (i, b) in vec.iter().enumerate() {
            data[i] = *b;
        }

        Some(data)
    }
}

pub trait AtlasPacketReader {
    fn get_packet(&self) -> Option<AtlasPacket>;
    fn get_string(&self, index: &mut usize) -> String;
    fn get_bool(&self, index: &mut usize) -> bool;
    fn get_u16(&self, index: &mut usize) -> u16;
}

impl AtlasPacketReader for [u8; ATLAS_PACKET_SIZE] {
    fn get_packet(&self) -> Option<AtlasPacket> {
        let mut index = 1;
        let t: AtlasPacketTypes = unsafe { ::std::mem::transmute(self[0]) };
        match t {
            AtlasPacketTypes::ConnectionRequest => return Some(
                AtlasPacket::ConnectionRequest,
            ),
            AtlasPacketTypes::ConnectionResponse => return Some(
                AtlasPacket::ConnectionResponse(self.get_string(&mut index))
            ),
            AtlasPacketTypes::LoginRequest => return Some(
                AtlasPacket::LoginRequest(self.get_string(&mut index), self.get_string(&mut index), self.get_string(&mut index))
            ),
            AtlasPacketTypes::LoginResponse => return Some(
                AtlasPacket::LoginResponse(self.get_u16(&mut index), self.get_string(&mut index))
            ),
            AtlasPacketTypes::RegisterRequest => return Some(
                AtlasPacket::RegisterRequest(self.get_string(&mut index), self.get_string(&mut index)),
            ),
            AtlasPacketTypes::RegisterResponse => return Some(
                AtlasPacket::RegisterResponse(self.get_u16(&mut index)),
            ),
            AtlasPacketTypes::JoinChatroomRequest => return Some(
                AtlasPacket::JoinChatroomRequest(self.get_string(&mut index), self.get_string(&mut index)),
            ),
            AtlasPacketTypes::JoinChatroomResponse => return Some(
                AtlasPacket::JoinChatroomResponse(self.get_u16(&mut index)),
            ),
            AtlasPacketTypes::SendMessageRequest => return Some(
                AtlasPacket::SendMessageRequest(self.get_string(&mut index), self.get_string(&mut index)),
            ),
            AtlasPacketTypes::SendMessageResponse => return Some(
                AtlasPacket::SendMessageResponse(self.get_u16(&mut index)),
            ),
            AtlasPacketTypes::RecvMessage => return Some(
                AtlasPacket::RecvMessage(self.get_string(&mut index), self.get_string(&mut index)),
            ),
        }
    }

    fn get_string(&self, index: &mut usize) -> String {
        let len_bytes: [u8; 8] = self[(*index)..(*index)+8].try_into().unwrap();
        *index += 8;
        let len = usize::from_be_bytes(len_bytes);
        let string = str::from_utf8(&self[(*index)..(*index) + len]).unwrap().to_string();
        *index += len;
        return string;
    }

    fn get_bool(&self, index: &mut usize) -> bool {
        let b: u8 = self[*index];
        *index += 1;
        return b == 1;
    }

    fn get_u16(&self, index: &mut usize) -> u16 {
        let bytes: [u8; 2] = self[(*index)..(*index)+2].try_into().unwrap();
        *index += 2;
        return u16::from_be_bytes(bytes);
    }
}