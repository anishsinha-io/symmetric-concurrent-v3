#![allow(unused_variables)]
use super::Replacer;

pub struct LruReplacer {}

impl Replacer for LruReplacer {
    fn pin(frame_id: crate::shared::FrameId) {}

    fn size() -> usize {
        0
    }

    fn unpin(frame_id: crate::shared::FrameId) {}

    fn victim(frame_id: crate::shared::FrameId) -> bool {
        false
    }
}
