#![allow(dead_code)]

pub type FrameId = isize;
pub type PageId = isize;
pub type Oid = u16;

pub const HEADER_ID: usize = 0;
pub const PAGE_SIZE: usize = 4096;
pub const INVALID_FRAME_ID: isize = -1;
pub const INVALID_PAGE_ID: isize = -1;
