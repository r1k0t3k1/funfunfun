use std::fmt::Display;

use uuid::Uuid;

macro_rules! id {
    ($($t:ident),*$(,)?) => {$(
        #[derive(Clone, Eq, PartialEq, Hash)] 
        pub struct $t(Uuid);

        impl $t {
            pub fn new() -> Self { Self(uuid::Uuid::new_v4()) }
            pub fn to_u128(&self) -> u128 { self.0.as_u128() }
        }

        impl From<Uuid> for $t {
            fn from(value: Uuid) -> Self {
                Self(value)
            }
        }

        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0.to_string())
            }
        }
    )*};
}

id!(
    OperatorId,
    ListenerId,
    AgentId,
);
