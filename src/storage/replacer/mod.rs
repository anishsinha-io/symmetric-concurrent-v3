pub mod lrukreplacer;
use crate::shared::FrameId;

pub(in crate::storage::replacer) trait Replacer {
    fn evict(frame_id: FrameId) -> bool;
    fn record_access(frame_id: FrameId);
    fn set_evictable(frame_id: FrameId, set_evictable: bool);
    fn remove(frame_id: FrameId);
}
