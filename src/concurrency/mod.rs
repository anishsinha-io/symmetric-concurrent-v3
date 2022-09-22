#![allow(dead_code)]
use parking_lot::lock_api::{
    RawMutex as _, RawRwLock as _, RawRwLockDowngrade, RawRwLockUpgrade, RawRwLockUpgradeDowngrade,
};

use std::sync::Arc;

/// Synchronized<T> allows a generic type to be thread safe and protected by a Mutex
pub type Synchronized<T> = Arc<parking_lot::Mutex<T>>;
/// RwSynchronized<T> allows a generic type to be thread safe and protected by a RwLock
pub type RwSynchronized<T> = Arc<parking_lot::RwLock<T>>;

#[inline]
pub unsafe fn acquire<T>(item: &Synchronized<T>) {
    item.raw().lock();
}

#[inline]
pub unsafe fn release<T>(item: &Synchronized<T>) {
    item.raw().unlock();
}

#[inline]
pub unsafe fn rw_acquire_shared<T>(item: &RwSynchronized<T>) {
    item.raw().lock_shared();
}

#[inline]
pub unsafe fn rw_acquire_upgradable<T>(item: &RwSynchronized<T>) {
    item.raw().lock_upgradable();
}

#[inline]
pub unsafe fn rw_acquire_excl<T>(item: &RwSynchronized<T>) {
    item.raw().lock_exclusive();
}

#[inline]
pub unsafe fn rw_upgrade_shared<T>(item: &RwSynchronized<T>) {
    item.raw().upgrade();
}

#[inline]
pub unsafe fn rw_release_shared<T>(item: &RwSynchronized<T>) {
    item.raw().unlock_shared();
}

#[inline]
pub unsafe fn rw_release_upgradable<T>(item: &RwSynchronized<T>) {
    item.raw().unlock_upgradable();
}

#[inline]
pub unsafe fn rw_release_excl<T>(item: &RwSynchronized<T>) {
    item.raw().unlock_exclusive();
}

#[inline]
pub unsafe fn rw_downgrade_excl_to_shared<T>(item: &RwSynchronized<T>) {
    item.raw().downgrade();
}

#[inline]
pub unsafe fn rw_downgrade_excl_to_upgradable<T>(item: &RwSynchronized<T>) {
    item.raw().downgrade_to_upgradable();
}

#[inline]
pub unsafe fn rw_downgrade_upgradable_to_shared<T>(item: &RwSynchronized<T>) {
    item.raw().downgrade_upgradable();
}
