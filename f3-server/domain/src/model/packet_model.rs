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
    pub listener_id: ListenerId,
    pub agent_pubkey: [u8; 32],
}

pub struct CheckinResponse {
    pub listener_id: ListenerId,
    pub agent_id: AgentId,
    pub listner_pubkey: [u8; 32],
}

pub trait PacketCodec {
    fn encode(raw_packet: Vec<u8>) -> Result<Packet, anyhow::Error>; 
    fn decode(packet: Packet) -> Result<Vec<u8>, anyhow::Error>; 
}
