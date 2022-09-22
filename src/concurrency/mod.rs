#![allow(dead_code)]
use parking_lot::lock_api::{
    RawRwLock as _, RawRwLockDowngrade, RawRwLockUpgrade, RawRwLockUpgradeDowngrade,
};

use std::sync::Arc;

pub type Synchronized<T> = Arc<parking_lot::RwLock<T>>;
pub enum LwLockType {
    Shared,
    SharedUpgradable,
    Exclusive,
}

#[inline]
pub unsafe fn acquire_shared<T>(item: &Synchronized<T>) {
    item.raw().lock_shared();
}

#[inline]
pub unsafe fn acquire_upgradable<T>(item: &Synchronized<T>) {
    item.raw().lock_upgradable();
}

#[inline]
pub unsafe fn acquire_excl<T>(item: &Synchronized<T>) {
    item.raw().lock_exclusive();
}

#[inline]
pub unsafe fn upgrade_shared<T>(item: &Synchronized<T>) {
    item.raw().upgrade();
}

#[inline]
pub unsafe fn release_shared<T>(item: &Synchronized<T>) {
    item.raw().unlock_shared();
}

#[inline]
pub unsafe fn release_upgradable<T>(item: &Synchronized<T>) {
    item.raw().unlock_upgradable();
}

#[inline]
pub unsafe fn release_excl<T>(item: &Synchronized<T>) {
    item.raw().unlock_exclusive();
}

#[inline]
pub unsafe fn downgrade_excl_to_shared<T>(item: &Synchronized<T>) {
    item.raw().downgrade();
}

#[inline]
pub unsafe fn downgrade_excl_to_upgradable<T>(item: &Synchronized<T>) {
    item.raw().downgrade_to_upgradable();
}

#[inline]
pub unsafe fn downgrade_upgradable_to_shared<T>(item: &Synchronized<T>) {
    item.raw().downgrade_upgradable();
}
