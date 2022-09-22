// SOURCES + USEFUL LINKS
// https://github.com/cmu-db/bustub/blob/master/src/include/storage/page/page.h

#![allow(dead_code, unused_imports)]
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::concurrency::{
    acquire_excl, acquire_shared, acquire_upgradable, release_excl, release_shared,
    release_upgradable, upgrade_shared, Synchronized,
};
use crate::shared::{PageId, PAGE_SIZE};

#[serde_as]
#[derive(Derivative, Deserialize, Serialize)]
#[derivative(Default)]
pub struct PageInternal {
    #[serde_as(as = "[_; PAGE_SIZE]")]
    #[derivative(Default(value = "[0u8; PAGE_SIZE]"))]
    data: [u8; PAGE_SIZE],
    #[derivative(Default(value = "-1"))]
    id: PageId,
    #[derivative(Default(value = "0"))]
    pin_count: usize,
    #[derivative(Default(value = "false"))]
    dirty: bool,
}

impl PageInternal {
    const PAGE_HEADER_SIZE: usize = 8;

    pub fn new() -> Self {
        PageInternal::default()
    }

    #[inline]
    pub fn get_data(&self) -> [u8; PAGE_SIZE] {
        self.data
    }

    #[inline]
    pub fn get_id(&self) -> PageId {
        self.id
    }

    pub fn get_pin_count(&self) -> usize {
        self.pin_count
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

pub type Page = Synchronized<PageInternal>;

#[inline]
pub fn w_latch(page: &Page) {
    unsafe {
        acquire_excl(&page);
    }
}

#[inline]
pub fn w_unlatch(page: &Page) {
    unsafe {
        release_excl(&page);
    }
}

#[inline]
pub fn r_latch(page: &Page) {
    unsafe {
        acquire_shared(&page);
    }
}

#[inline]
pub fn r_unlatch(page: &Page) {
    unsafe {
        release_shared(&page);
    }
}

#[inline]
pub fn u_latch(page: &Page) {
    unsafe {
        acquire_upgradable(&page);
    }
}

#[inline]
pub fn u_unlatch(page: &Page) {
    unsafe {
        release_upgradable(&page);
    }
}

#[inline]
pub fn u_upgrade_latch(page: &Page) {
    unsafe {
        upgrade_shared(&page);
    }
}
