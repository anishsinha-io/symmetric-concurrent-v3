#![allow(unused_imports, unused_variables)]

use crate::{
    concurrency::{RwSynchronized, Synchronized},
    shared::FrameId,
};

use crate::storage::replacer::Replacer;

pub struct LRUKReplacerInternal {}

impl LRUKReplacerInternal {}

impl Replacer for LRUKReplacerInternal {
    fn evict(frame_id: FrameId) -> bool {
        return false;
    }
    fn record_access(frame_id: FrameId) {}
    fn set_evictable(frame_id: FrameId, set_evictable: bool) {}
    fn remove(frame_id: FrameId) {}
}

pub type LRUKReplacer = Synchronized<LRUKReplacerInternal>;
