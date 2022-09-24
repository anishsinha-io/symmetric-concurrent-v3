#![allow(dead_code, unused_imports)]

use std::collections::{HashMap, LinkedList};

use crate::concurrency::RwSynchronized;
use crate::shared::{FrameId, PageId};
use crate::storage::diskmgr::DiskMgr;
use crate::storage::page_table::PageTable;

struct BufferPoolInternal {
    diskmgr: DiskMgr,
    page_table: PageTable,
    free_list: LinkedList<FrameId>,
}

#[cfg(test)]
mod tests {
    // #[test]
    fn create() {}
}
