use anyhow::anyhow;
use application::outbound::agent::{AgentEvent, AgentId};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, aead::{Aead, KeyInit, Payload}};
use rand::Rng;
use serde::{Deserialize, Serialize};

const MAGIC_NUMBER: u16 = 0xf3f3;
const NONCE_LEN: usize = 12;
const LENGTH_LEN: usize = 8;
const AAD_LEN: usize = NONCE_LEN + LENGTH_LEN;
const TAG_LEN: usize = 16;

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub magic: u16,
    pub agent_id: [u8;16],
    pub length: u64,
    pub body: Body,
}

impl Packet {
    pub fn new(inner_packet: Vec<Tlv>, agent_id: AgentId) -> Self {
        Self { 
            magic: MAGIC_NUMBER,
            agent_id: agent_id.into_bytes(),
            length: serde_cbor::to_vec(&inner_packet).unwrap().len() as u64, // TODO
            body: Body::Plain(inner_packet),
        }
    }


    pub fn encrypt(&mut self, key: [u8; 32]) -> Result<(), anyhow::Error>{
        match &self.body {
            Body::Encrypted {nonce: _, cipher_text: _, tag: _} => return Err(anyhow::anyhow!("Packet already encrypted")),
            Body::Plain(plain_tlv) => {
                let mut nonce = [0_u8; NONCE_LEN];
                rand::rng().fill_bytes(&mut nonce);
                let plain = serde_cbor::to_vec(&plain_tlv)
                    .map_err(|e| anyhow::anyhow!("{e}"))?;


                let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
                let nonce = Nonce::from_slice(&nonce); // 12 バイト
                println!("plain.len() = {}", plain.len());
                let cipher_text = cipher.encrypt(nonce, Payload { msg: &plain, aad: &[] })
                    .map_err(|e| anyhow::anyhow!("encrypt failed: {e}"))?;
                self.body = Body::Encrypted {
                    nonce: Into::<[u8; 12]>::into(*nonce),
                    cipher_text,
                    tag: [0_u8; 16],
                };
            },
        };

        Ok(())
    }
    
    pub fn decrypt(&mut self, key: [u8; 32]) -> Result<Vec<Tlv>, anyhow::Error> {
        match &self.body {
            Body::Plain(_) => return Err(anyhow::anyhow!("Packet already decrypted")),
            Body::Encrypted { nonce, cipher_text, tag: _ } => {
                let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));

                let plain = cipher.decrypt(nonce.into(), cipher_text.as_ref())
                    .map_err(|e| anyhow::anyhow!("decrypt failed: {e}"))?;

                let tlv = serde_cbor::from_slice(&plain)
                    .map_err(|e| anyhow::anyhow!("{e}"))?;
                Ok(tlv)
            }
        }
    }
}

impl TryInto<Vec<AgentEvent>> for Packet {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<AgentEvent>, Self::Error> {
        match self.body {
            Body::Encrypted { nonce: _, cipher_text: _, tag: _ } => Err(anyhow!("Packet is Encrypted")),
            Body::Plain(tlvs) => {
                let mut results = vec![];
                for tlv in tlvs {
                    results.push(tlv.try_into()?)
                }
                Ok(results)
            }
        } 
    }
}

impl TryInto<AgentEvent> for Tlv {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<AgentEvent, Self::Error> {
        match self {
            Self::CheckinReq(v) => Ok(AgentEvent::Checkin { agent_public_key: v.agent_pubkey, response_sender: todo!() }),
            Self::CheckinCompleteReq(v) => Ok(AgentEvent::CheckinComplete { agent_info: v.agent_info, response_sender: todo!() }),
            _ => Err(anyhow!("Cannot convert Tlv to Agent Event: {self:?}")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Body {
    Plain(Vec<Tlv>),
    Encrypted {
        nonce: [u8; NONCE_LEN],
        cipher_text: Vec<u8>,
        tag: [u8; TAG_LEN],
    },
}

impl Body {
    pub fn associated_data(&self) -> Vec<u8> {
        match self {
            Body::Plain(_) => vec![],
            Body::Encrypted { nonce, cipher_text: _, tag: _ } => {
                let mut out = [0u8; NONCE_LEN];
                out[..NONCE_LEN].copy_from_slice(nonce);
                out.to_vec()
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Tlv {
    CheckinReq(CheckinRequest),
    CheckinRes(CheckinResponse),
    CheckinCompleteReq(CheckinCompleteRequest),
    CheckinCompleteRes(CheckinCompleteResponse),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckinRequest { // ここのみ平文
    pub agent_pubkey: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckinResponse { // ここのみ平文
    pub listener_pubkey: [u8; 32],
}

impl CheckinResponse {
    pub fn new(listener_pubkey: [u8; 32]) -> Self {
        Self { listener_pubkey } 
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckinCompleteRequest { 
    pub agent_id: String, 
    pub agent_info: String, // 共通鍵で暗号化したホストの情報などを送る // TODO
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckinCompleteResponse { // ↑が復号できればCheckin完了、サーバ側でAgent登録
    pub listener_id: String,
    pub agent_id: String,
}

impl CheckinCompleteResponse {
    pub fn new(listener_id: String, agent_id: String) -> Self {
        Self { listener_id, agent_id } 
    }
}
