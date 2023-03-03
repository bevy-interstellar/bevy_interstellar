use super::oid::Oid;
use bincode::serde::encode_into_std_write;
use serde::Serialize;

pub trait Property: Serialize {
    fn id(&self) -> Oid {
        Oid::v5_from_write(|w| {
            encode_into_std_write(&self, w, crate::utils::SERDE_CONFIG).unwrap();
        })
    }
}
