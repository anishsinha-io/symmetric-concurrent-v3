/// This file implements a file API utilized primarily by the disk manager
use std::fs::File;
use std::io::SeekFrom;

use crate::shared::PAGE_SIZE;

/// Used to write a buffer to a specified offset in the file handle passed in
pub fn write_bytes(mut handle: &File, bytes: &[u8; PAGE_SIZE], offset: u64) -> std::io::Result<()> {
    use std::io::prelude::*;
    handle.seek(SeekFrom::Start(offset))?;
    handle.write(bytes)?;
    Ok(())
}

/// Used to read from a specified offset, enough bytes to fill the passed in buffer
pub fn read_bytes(
    mut handle: &File,
    buffer: &mut [u8; PAGE_SIZE],
    offset: u64,
) -> std::io::Result<()> {
    use std::io::prelude::*;
    handle.seek(SeekFrom::Start(offset))?;
    handle.read(buffer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::fs::OpenOptions;
    use std::sync::Arc;

    use super::*;
    use crate::concurrency::{acquire, release, Synchronized};
    use crate::shared::Song;
    use crate::storage::ioutil::{from_buffer, to_buffer};

    const FSUTIL_TEST_PATH: &'static str =
        "/Users/anishsinha/Home/personal/research/symmetric-concurrent/symmetric-concurrent-v3/data/test/__fsutil__/fsutil.bin";

    lazy_static! {
        /// Synchronized file handle for use in testing. It needs to be synchronized because Rust tests are run in parallel
        static ref TEST_FILE_HANDLE: Synchronized<File> = Arc::new(parking_lot::Mutex::new(
            OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .truncate(true)
                .open(std::path::Path::new(FSUTIL_TEST_PATH))
                .unwrap()
        ));
    }

    #[test]
    fn read_write_buffer() {
        // Need an unsafe block to use acquire/release.
        // TODO refactor to use a RAII guard. Unsafe acquire/release should only be used when latching internal data structures
        unsafe {
            acquire(&TEST_FILE_HANDLE);
            let handle = &(*TEST_FILE_HANDLE.data_ptr());
            let cry_baby = Song::new(1, "Cry Baby", "The Neighbourhood");
            let cry_baby_buf = to_buffer(cry_baby).unwrap();
            let paris = Song::new(2, "Paris", "The 1975");
            let paris_buf = to_buffer(paris).unwrap();
            let tangerine = Song::new(
                3,
                "Tangerine (feat Arlo Parks)",
                "Glass Animals, Arlo Parks",
            );
            let tangerine_buf = to_buffer(tangerine).unwrap();

            handle.set_len(0).unwrap();
            assert!(!write_bytes(
                &handle,
                &cry_baby_buf,
                (cry_baby.id as u64 - 1u64) * PAGE_SIZE as u64
            )
            .is_err());
            assert!(!write_bytes(
                &handle,
                &paris_buf,
                (paris.id as u64 - 1u64) * PAGE_SIZE as u64
            )
            .is_err());
            assert!(!write_bytes(
                &handle,
                &tangerine_buf,
                (tangerine.id as u64 - 1u64) * PAGE_SIZE as u64
            )
            .is_err());

            let mut decoded_cry_baby_buf = [0u8; PAGE_SIZE];
            let decoded_cry_baby_read_result = read_bytes(
                &handle,
                &mut decoded_cry_baby_buf,
                (cry_baby.id as u64 - 1) * PAGE_SIZE as u64,
            );
            assert!(!decoded_cry_baby_read_result.is_err());
            let decoded_cry_baby = from_buffer::<Song>(&decoded_cry_baby_buf).unwrap();
            assert_eq!(cry_baby.id, decoded_cry_baby.id);
            assert_eq!(cry_baby.title, decoded_cry_baby.title);
            assert_eq!(cry_baby.artist, decoded_cry_baby.artist);

            let mut decoded_paris_buf = [0u8; PAGE_SIZE];
            let decoded_paris_read_result = read_bytes(
                &handle,
                &mut decoded_paris_buf,
                (paris.id as u64 - 1) * PAGE_SIZE as u64,
            );
            assert!(!decoded_paris_read_result.is_err());
            let decoded_paris = from_buffer::<Song>(&decoded_paris_buf).unwrap();

            assert_eq!(paris.id, decoded_paris.id);
            assert_eq!(paris.title, decoded_paris.title);
            assert_eq!(paris.artist, decoded_paris.artist);

            let mut decoded_tangerine_buf = [0u8; PAGE_SIZE];
            let decoded_tangerine_read_result = read_bytes(
                &handle,
                &mut decoded_tangerine_buf,
                (tangerine.id as u64 - 1) * PAGE_SIZE as u64,
            );
            assert!(!decoded_tangerine_read_result.is_err());
            let decoded_tangerine = from_buffer::<Song>(&decoded_tangerine_buf).unwrap();

            assert_eq!(tangerine.id, decoded_tangerine.id);
            assert_eq!(tangerine.title, decoded_tangerine.title);
            assert_eq!(tangerine.artist, decoded_tangerine.artist);

            handle.set_len(0).unwrap();
            release(&TEST_FILE_HANDLE);
        }
    }

    #[test]
    fn read_write_unordered() {
        // Need an unsafe block to use acquire/release.
        // TODO refactor to use a RAII guard. Unsafe acquire/release should only be used when latching internal data structures
        unsafe {
            acquire(&TEST_FILE_HANDLE);
            let handle = &(*TEST_FILE_HANDLE.data_ptr());
            let you_found_me = Song::new(1, "You Found Me", "The Fray");

            handle.set_len(0).unwrap();
            let you_found_me_buf = to_buffer(you_found_me).unwrap();

            assert!(!write_bytes(
                &handle,
                &you_found_me_buf,
                (you_found_me.id as u64) * PAGE_SIZE as u64
            )
            .is_err());

            let mut decoded_you_found_me_buf = [0u8; PAGE_SIZE];

            let decoded_you_found_me_read_result = read_bytes(
                &handle,
                &mut decoded_you_found_me_buf,
                (you_found_me.id as u64) * PAGE_SIZE as u64,
            );
            assert!(!decoded_you_found_me_read_result.is_err());
            let decoded_you_found_me = from_buffer::<Song>(&decoded_you_found_me_buf).unwrap();

            assert_eq!(you_found_me.id, decoded_you_found_me.id);
            assert_eq!(you_found_me.title, decoded_you_found_me.title);
            assert_eq!(you_found_me.artist, decoded_you_found_me.artist);

            handle.set_len(0).unwrap();
            release(&TEST_FILE_HANDLE);
        }
    }
}
