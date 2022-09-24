#![allow(dead_code, unused_imports)]

use std::fs::File;
use std::sync::Arc;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::concurrency::{acquire, release, Synchronized};
use crate::storage::fsutil::{read_bytes, write_bytes};
use crate::storage::ioutil::{decode, encode, from_buffer, to_buffer};

pub struct DiskMgr {
    db_handle: Synchronized<File>,
    log_handle: Synchronized<File>,
}

impl DiskMgr {}
