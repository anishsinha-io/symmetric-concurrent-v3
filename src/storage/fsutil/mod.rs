#![allow(unused_imports)]
use std::fs::File;
use std::io::SeekFrom;

use crate::shared::PAGE_SIZE;
use crate::storage::ioutil::{decode, encode, from_buffer, to_buffer};

pub fn write_bytes(mut handle: File, bytes: [u8; PAGE_SIZE], offset: u64) -> std::io::Result<()> {
    use std::io::prelude::*;
    handle.seek(SeekFrom::Start(offset))?;
    handle.write(&bytes)?;
    Ok(())
}

pub fn read_bytes(
    mut handle: File,
    buffer: &mut [u8; PAGE_SIZE],
    offset: u64,
) -> std::io::Result<()> {
    use std::io::prelude::*;
    handle.seek(SeekFrom::Start(offset))?;
    handle.read(buffer)?;
    Ok(())
}
