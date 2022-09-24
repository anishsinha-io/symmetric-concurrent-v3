#![allow(dead_code, unused_imports)]

use std::borrow::BorrowMut;
use std::collections::{HashMap, LinkedList};
use std::sync::Arc;

use crate::concurrency::RwSynchronized;
use crate::shared::{FrameId, PageId};
use crate::storage::diskmgr::DiskMgr;
use crate::storage::page_table::PageTable;

use super::diskmgr::DiskMgrInternal;

pub type FreeList<T> = RwSynchronized<LinkedList<T>>;

pub struct BufferPoolInternal {
    pool_size: usize,
    replacer_k: usize,
    diskmgr: DiskMgr,
    page_table: PageTable,
    free_list: FreeList<FrameId>,
}

impl BufferPoolInternal {
    pub fn new(pool_size: usize, replacer_k: usize, diskmgr: DiskMgr) -> Self {
        let mut free_list_internal: LinkedList<FrameId> = LinkedList::new();

        for i in 0..pool_size {
            free_list_internal.push_back(i as isize);
        }
        Self {
            pool_size,
            replacer_k,
            diskmgr,
            page_table: Arc::new(parking_lot::RwLock::new(HashMap::new())),
            free_list: Arc::new(parking_lot::RwLock::new(free_list_internal)),
        }
    }
}

pub type BufferPool = RwSynchronized<BufferPoolInternal>;

#[cfg(test)]
mod tests {
    // #[test]
    fn create() {}
}
