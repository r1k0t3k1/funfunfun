pub struct Agent {
    pub agent_id: String,
    pub listener_id: String,
}

type AgentPublicKey = [u8; 32];

pub enum AgentEvent {
    Checkin { agent_public_key: AgentPublicKey},
    CheckinComplete { agent_info: String }, // TODO
}
