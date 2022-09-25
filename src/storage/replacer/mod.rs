pub mod lrureplacer;

use crate::shared::FrameId;

pub(in crate::storage::replacer) trait Replacer {
    fn victim(frame_id: FrameId) -> bool;
    fn pin(frame_id: FrameId);
    fn unpin(frame_id: FrameId);
    fn size() -> usize;
}
