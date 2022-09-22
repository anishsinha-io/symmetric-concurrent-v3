use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn encode<T>(item: T) -> Option<Vec<u8>>
where
    T: Sized + Serialize,
{
    let encoded = bincode::serialize(&item);
    match encoded {
        Ok(encoded) => Some(encoded),
        Err(_) => None,
    }
}

pub fn decode<T>(bytes: Vec<u8>) -> Option<T>
where
    T: Sized + Serialize + DeserializeOwned,
{
    let decoded = bincode::deserialize(&bytes[..]);
    match decoded {
        Ok(decoded) => Some(decoded),
        Err(_) => None,
    }
}

pub trait ReadWriteCloser {
    fn encode(&self) -> Vec<u8>
    where
        Self: Sized + Serialize;
    fn decode(bytes: Vec<u8>) -> Option<Self>
    where
        Self: Sized + Serialize + DeserializeOwned;
}
