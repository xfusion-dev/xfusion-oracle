use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use candid::Principal;
use crate::types::{Symbol, Price, Policy};
use crate::ring_buffer::RingBuffer;
use crate::archive::Archive;

const HISTORY_CAPACITY: usize = 2880; // 30 days * 24 hours * 4 samples/hour

thread_local! {
    static PRICE_STORAGE: RefCell<PriceStorage> = RefCell::new(PriceStorage::new());
}

pub struct PriceStorage {
    pub prices: HashMap<Symbol, Price>,
    pub history: HashMap<Symbol, RingBuffer<Price>>,
    pub archives: HashMap<Symbol, Archive>,
    pub symbols: HashSet<Symbol>,
    pub policy: Policy,
    pub version: u64,
    pub allowed_updaters: HashSet<Principal>,
    pub managers: HashSet<Principal>,
}

impl PriceStorage {
    pub fn new() -> Self {
        Self {
            prices: HashMap::new(),
            history: HashMap::new(),
            archives: HashMap::new(),
            symbols: HashSet::new(),
            policy: Policy {
                aggregation: "LAST".to_string(),
                retain_history: 30,
            },
            version: 0,
            allowed_updaters: HashSet::new(),
            managers: HashSet::new(),
        }
    }

    pub fn add_price_with_history(&mut self, symbol: Symbol, price: Price) {
        let price_clone = price.clone();
        self.prices.insert(symbol.clone(), price);

        self.history
            .entry(symbol)
            .or_insert_with(|| RingBuffer::new(HISTORY_CAPACITY))
            .push(price_clone);
    }
}

pub fn with_storage<R>(f: impl FnOnce(&PriceStorage) -> R) -> R {
    PRICE_STORAGE.with(|storage| f(&storage.borrow()))
}

pub fn with_storage_mut<R>(f: impl FnOnce(&mut PriceStorage) -> R) -> R {
    PRICE_STORAGE.with(|storage| f(&mut storage.borrow_mut()))
}