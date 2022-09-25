use crate::concurrency::RwSynchronized;
use std::collections::LinkedList;

pub type FreeList<T> = RwSynchronized<LinkedList<T>>;
