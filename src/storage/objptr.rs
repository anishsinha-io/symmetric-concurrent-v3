#[allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectPtr {
    loc: usize,
}

impl ObjectPtr {
    pub fn new() -> Self {
        ObjectPtr { loc: 0 }
    }
}

impl Display for ObjectPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nObjectPtr [loc={}]", self.loc)
    }
}
