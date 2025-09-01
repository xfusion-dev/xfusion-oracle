use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use candid::Principal;
use crate::types::{Symbol, Price};

thread_local! {
    static PRICE_STORAGE: RefCell<PriceStorage> = RefCell::new(PriceStorage::new());
}

pub struct PriceStorage {
    pub prices: HashMap<Symbol, Price>,
    pub version: u64,
    pub allowed_updaters: HashSet<Principal>,
    pub managers: HashSet<Principal>,
}

impl PriceStorage {
    pub fn new() -> Self {
        Self {
            prices: HashMap::new(),
            version: 0,
            allowed_updaters: HashSet::new(),
            managers: HashSet::new(),
        }
    }
}

pub fn with_storage<R>(f: impl FnOnce(&PriceStorage) -> R) -> R {
    PRICE_STORAGE.with(|storage| f(&storage.borrow()))
}

pub fn with_storage_mut<R>(f: impl FnOnce(&mut PriceStorage) -> R) -> R {
    PRICE_STORAGE.with(|storage| f(&mut storage.borrow_mut()))
}