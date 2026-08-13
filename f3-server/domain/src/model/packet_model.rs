use rand::Rng;

pub type ListenerId = String;
pub type AgentId = String;

struct Packet {
    pub magic: u16,
    pub r#type: u8,
    pub length: u16,
    pub payload: Vec<Tlv>,
}

pub enum Tlv {
    CheckinReq { length: usize, value: CheckinRequest },
    CheckinRes { length: usize, value: CheckinResponse },
}

pub struct CheckinRequest {
    pub agent_pubkey: [u8; 32],
}

#[derive(Debug)]
pub struct CheckinResponse {
    pub listener_pubkey: [u8; 32],
}

impl CheckinResponse {
    pub fn new() -> Self {
        let mut listener_pubkey = [0_u8; 32];
        rand::rng().fill_bytes(&mut listener_pubkey);
        Self { listener_pubkey }
    }
}

pub struct CheckinCompleteRequest {
    pub agent_info: String,
}

pub struct CheckinCompleteResponse {
    pub listener_id: ListenerId,
    pub agent_id: AgentId,
}

pub trait PacketCodec {
    fn encode(raw_packet: Vec<u8>) -> Result<Packet, anyhow::Error>; 
    fn decode(packet: Packet) -> Result<Vec<u8>, anyhow::Error>; 
}
