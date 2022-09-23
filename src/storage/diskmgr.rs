#![allow(dead_code, unused_imports, unused_variables)]
use serde::{de::DeserializeOwned, Serialize};

use crate::concurrency::Synchronized;
use crate::shared::{PageId, PAGE_SIZE};
use crate::storage::ioutil::{decode, encode};

use std::fs::File;

pub trait IOCloser {
    fn encode<T>(item: T) -> Option<Vec<u8>>
    where
        T: Sized + Serialize;
    fn decode<T>(bytes: Vec<u8>) -> Option<T>
    where
        T: Sized + Serialize + DeserializeOwned;
    fn db_read_bytes(&self, id: PageId, page: [u8; PAGE_SIZE]) -> std::io::Result<()>;
    fn db_write_bytes(&self, id: PageId, page: [u8; PAGE_SIZE]) -> std::io::Result<()>;
}

pub struct DiskMgrInternal {
    file_path: String,
    db_handle: File,
    log_handle: File,
    num_writes: usize,
    num_flushes: usize,
}

impl DiskMgrInternal {
    fn get_file_size(file_name: &str) -> Option<u64> {
        use std::fs;
        if let Ok(file) = fs::metadata(file_name) {
            return Some(file.len());
        }
        None
    }
}

impl IOCloser for DiskMgrInternal {
    fn decode<T>(bytes: Vec<u8>) -> Option<T>
    where
        T: Sized + DeserializeOwned + Serialize,
    {
        if let Ok(decoded) = bincode::deserialize(&bytes[..]) {
            return Some(decoded);
        }
        None
    }

    fn encode<T>(item: T) -> Option<Vec<u8>>
    where
        T: Sized + Serialize,
    {
        if let Ok(encoded) = bincode::serialize(&item) {
            return Some(encoded);
        }
        None
    }

    fn db_read_bytes(&self, id: PageId, mut page: [u8; PAGE_SIZE]) -> std::io::Result<()> {
        use std::io::prelude::*;
        use std::io::SeekFrom;
        let mut file = &self.db_handle;
        file.seek(SeekFrom::Start(id as u64 * PAGE_SIZE as u64))?;
        file.read(&mut page)?;
        Ok(())
    }

    fn db_write_bytes(&self, id: PageId, page: [u8; PAGE_SIZE]) -> std::io::Result<()> {
        use std::io::prelude::*;
        use std::io::SeekFrom;
        let mut file = &self.db_handle;
        file.seek(SeekFrom::Start(id as u64 * PAGE_SIZE as u64))?;
        file.write(&page)?;
        Ok(())
    }
}

pub type DiskMgr = Synchronized<DiskMgrInternal>;
