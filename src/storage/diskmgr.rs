#![allow(unused_imports)]
use std::fs::{File, OpenOptions};
use std::sync::Arc;

use crate::concurrency::Synchronized;
use crate::shared::{PageId, PAGE_SIZE};
use crate::storage::fsutil::{read_bytes, write_bytes};

pub struct DiskMgrInternal {
    file_handle: Synchronized<File>,
    file_path: String,
    num_flushes: usize,
    num_writes: usize,
}

impl DiskMgrInternal {
    pub fn new(file_path: &str) -> Self {
        let file_handle: Synchronized<File> = Arc::new(parking_lot::Mutex::new(
            OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .truncate(true)
                .open(std::path::Path::new(file_path))
                .unwrap(),
        ));

        Self {
            file_handle,
            file_path: String::from(file_path),
            num_flushes: 0,
            num_writes: 0,
        }
    }

    /// Shutdown DiskMgr and close all open file handles
    pub fn close(&self) {
        drop(self);
    }

    pub fn write_page(&self, id: PageId, page_buf: &[u8; PAGE_SIZE]) -> std::io::Result<()> {
        let file = self.file_handle.lock();
        write_bytes(&file, page_buf, PAGE_SIZE as u64 * id as u64)?;
        Ok(())
    }

    pub fn read_page(&self, id: PageId, page_buf: &mut [u8; PAGE_SIZE]) -> std::io::Result<()> {
        let file = self.file_handle.lock();
        read_bytes(&file, page_buf, PAGE_SIZE as u64 * id as u64)?;
        Ok(())
    }
}

pub type DiskMgr = Arc<parking_lot::RwLock<DiskMgrInternal>>;

#[cfg(test)]
mod tests {
    use super::*;
}
