use std::fmt::Display;

use uuid::Uuid;

pub trait Id: From<Uuid> + Display {
    fn new(id: Uuid) -> Self;
    fn to_u128(&self) -> u128;
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct OperatorId(Uuid);

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ListenerId(Uuid);

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct AgentId(Uuid);

impl Id for OperatorId {
    fn new(id: Uuid) -> Self { Self(id) }
    fn to_u128(&self) -> u128 { self.0.as_u128() }
}
impl From<Uuid> for OperatorId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}
impl Display for OperatorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl Id for ListenerId {
    fn new(id: Uuid) -> Self { Self(id) }
    fn to_u128(&self) -> u128 { self.0.as_u128() }
}
impl From<Uuid> for ListenerId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}
impl Display for ListenerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl Id for AgentId {
    fn new(id: Uuid) -> Self { Self(id) }
    fn to_u128(&self) -> u128 { self.0.as_u128() }
}
impl From<Uuid> for AgentId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}
impl Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
