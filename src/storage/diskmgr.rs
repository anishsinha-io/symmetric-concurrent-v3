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
        // sync filesystem
        file.sync_all()?;
        Ok(())
    }

    pub fn read_page(&self, id: PageId, page_buf: &mut [u8; PAGE_SIZE]) -> std::io::Result<()> {
        let file = self.file_handle.lock();
        read_bytes(&file, page_buf, PAGE_SIZE as u64 * id as u64)?;
        Ok(())
    }

    pub fn clear(&self) -> std::io::Result<()> {
        unsafe {
            (*self.file_handle.data_ptr()).set_len(0)?;
        }
        Ok(())
    }
}

pub type DiskMgr = Arc<parking_lot::RwLock<DiskMgrInternal>>;

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;
    use rayon::ThreadPoolBuilder;
    use std::sync::Arc;

    use crate::concurrency::{
        acquire, rw_acquire_excl, rw_acquire_shared, rw_release_excl, rw_release_shared,
    };
    use crate::shared::Song;
    use crate::storage::ioutil;

    const DISKMGR_TEST_PATH: &'static str = "/Users/anishsinha/Home/personal/research/symmetric-concurrent/symmetric-concurrent-v3/data/test/__diskmgr__/diskmgr.bin";

    lazy_static! {
        static ref DISKMGR: DiskMgr = Arc::new(parking_lot::RwLock::new(DiskMgrInternal::new(
            DISKMGR_TEST_PATH
        )));
        /// Used in test threaded_rw
        static ref DONE_STATE: Arc<(parking_lot::Mutex<bool>, parking_lot::Condvar)> =
            Arc::new((parking_lot::Mutex::new(false), parking_lot::Condvar::new()));
    }

    #[test]
    fn rw() {
        let internal = unsafe { &(*DISKMGR.data_ptr()) };
        let helium = Song::new(1, "Helium", "Glass Animals");
        let helium_buf = ioutil::to_buffer(helium).unwrap();
        assert!(!internal.clear().is_err());
        assert!(!internal
            .write_page(helium.id as isize, &helium_buf)
            .is_err());
        let mut helium_disk_buf = [0u8; PAGE_SIZE];
        assert!(!internal
            .read_page(helium.id as isize, &mut helium_disk_buf)
            .is_err());
        let helium_from_buf = ioutil::from_buffer::<Song>(&helium_disk_buf).unwrap();

        assert_eq!(helium.id, helium_from_buf.id);
        assert_eq!(helium.title, helium_from_buf.title);
        assert_eq!(helium.artist, helium_from_buf.artist);
    }
}
