use crate::{
    concurrency::RwSynchronized,
    shared::{FrameId, PageId},
};
use std::collections::HashMap;

/// Thread safe HashMap (protected by a parking_lot::RwLock)
/// Just lock it and then perform operations normally.
pub type PageTable = RwSynchronized<HashMap<PageId, FrameId>>;
