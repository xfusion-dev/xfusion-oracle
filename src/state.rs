use std::collections::HashMap;
use std::cell::RefCell;
use crate::types::{Symbol, Price};

thread_local! {
    static PRICE_STORAGE: RefCell<PriceStorage> = RefCell::new(PriceStorage::new());
}

pub struct PriceStorage {
    pub prices: HashMap<Symbol, Price>,
    pub version: u64,
}

impl PriceStorage {
    pub fn new() -> Self {
        Self {
            prices: HashMap::new(),
            version: 0,
        }
    }
}

pub fn with_storage<R>(f: impl FnOnce(&PriceStorage) -> R) -> R {
    PRICE_STORAGE.with(|storage| f(&storage.borrow()))
}

pub fn with_storage_mut<R>(f: impl FnOnce(&mut PriceStorage) -> R) -> R {
    PRICE_STORAGE.with(|storage| f(&mut storage.borrow_mut()))
}