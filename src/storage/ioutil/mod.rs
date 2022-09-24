use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::shared::PAGE_SIZE;

pub fn encode<T>(item: T) -> Option<Vec<u8>>
where
    T: Sized + Serialize,
{
    if let Ok(encoded) = bincode::serialize(&item) {
        return Some(encoded);
    }
    None
}

pub fn decode<T>(bytes: Vec<u8>) -> Option<T>
where
    T: Sized + Serialize + DeserializeOwned,
{
    if let Ok(decoded) = bincode::deserialize(&bytes) {
        return Some(decoded);
    }
    None
}

pub fn to_buffer<T>(item: T) -> Option<[u8; PAGE_SIZE]>
where
    T: Sized + Serialize,
{
    if let Some(encoded) = encode(item) {
        let mut buf = [0u8; PAGE_SIZE];
        buf[..std::mem::size_of_val(&*encoded)].copy_from_slice(&*encoded);
        return Some(buf);
    }
    None
}

pub fn from_buffer<T>(buf: [u8; PAGE_SIZE]) -> Option<T>
where
    T: Sized + Serialize + DeserializeOwned,
{
    decode::<T>(buf.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use derivative::Derivative;
    use serde::Deserialize;
    use serde_with::serde_as;

    #[serde_as]
    #[derive(Serialize, Deserialize, Derivative, Clone, Copy)]
    #[derivative(Default)]
    pub struct Song {
        // the default id value (-1) denotes that the struct is invalid. all valid songs must have a positive id
        #[derivative(Default(value = "-1"))]
        pub id: i32,
        #[derivative(Default(value = "[0u8; 50]"))]
        #[serde_as(as = "[_; 50]")]
        pub title: [u8; 50],
        #[derivative(Default(value = "[0u8; 50]"))]
        #[serde_as(as = "[_; 50]")]
        pub artist: [u8; 50],
    }

    impl Song {
        pub fn new<'a>(id: i32, title: &'a str, artist: &'a str) -> Song {
            let mut song_buf = [0u8; 50];
            song_buf[..title.len()].copy_from_slice(title.as_bytes());
            let mut artist_buf = [0u8; 50];
            artist_buf[..artist.len()].copy_from_slice(artist.as_bytes());
            return Song {
                id,
                title: song_buf,
                artist: artist_buf,
            };
        }
    }

    #[test]
    fn encode_decode() {
        let cry_baby = Song::new(1, "Cry Baby", "The Neighbourhood");
        let bytes = encode(cry_baby).unwrap();
        let decoded = decode::<Song>(bytes).unwrap();

        assert_eq!(cry_baby.id, decoded.id);
        assert_eq!(cry_baby.artist, decoded.artist);
        assert_eq!(cry_baby.title, decoded.title);
    }

    #[test]
    fn encode_decode_buffer() {
        let cry_baby = Song::new(1, "Cry Baby", "The Neighbourhood");
        let buf = to_buffer(cry_baby).unwrap();
        let decoded = from_buffer::<Song>(buf).unwrap();

        assert_eq!(cry_baby.id, decoded.id);
        assert_eq!(cry_baby.artist, decoded.artist);
        assert_eq!(cry_baby.title, decoded.title);
    }
}
