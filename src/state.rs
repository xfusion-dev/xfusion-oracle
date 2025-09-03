use std::collections::HashMap;
use std::cell::RefCell;
use candid::Principal;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::memory_manager::{MemoryManager, MemoryId, VirtualMemory};
use crate::types::{Symbol, Price, Policy, Bar};
use crate::ring_buffer::RingBuffer;
use crate::archive::Archive;
use crate::ohlc::OHLCBuilder;

const HISTORY_CAPACITY: usize = 2880; // 30 days * 24 hours * 4 samples/hour

// Memory IDs for different stable structures
const PRICES_MEMORY_ID: MemoryId = MemoryId::new(0);
const SYMBOLS_MEMORY_ID: MemoryId = MemoryId::new(1);
const MANAGERS_MEMORY_ID: MemoryId = MemoryId::new(2);
const UPDATERS_MEMORY_ID: MemoryId = MemoryId::new(3);

type MemoryType = VirtualMemory<DefaultMemoryImpl>;

impl Storable for Price {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let bytes = candid::encode_one(self).expect("Failed to encode Price");
        std::borrow::Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).expect("Failed to decode Price")
    }

    const BOUND: Bound = Bound::Unbounded;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static PRICES: RefCell<StableBTreeMap<String, Price, MemoryType>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(PRICES_MEMORY_ID)),
        )
    );

    static SYMBOLS: RefCell<StableBTreeMap<String, (), MemoryType>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(SYMBOLS_MEMORY_ID)),
        )
    );

    static MANAGERS: RefCell<StableBTreeMap<String, (), MemoryType>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MANAGERS_MEMORY_ID)),
        )
    );

    static UPDATERS: RefCell<StableBTreeMap<String, (), MemoryType>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(UPDATERS_MEMORY_ID)),
        )
    );

    static PRICE_STORAGE: RefCell<PriceStorage> = RefCell::new(PriceStorage::new());
}

pub struct PriceStorage {
    pub history: HashMap<Symbol, RingBuffer<Price>>,
    pub archives: HashMap<Symbol, Archive>,
    pub ohlc_builders: HashMap<Symbol, HashMap<String, OHLCBuilder>>, // symbol -> resolution -> builder
    pub policy: Policy,
    pub version: u64,
    pub total_updates: u64,
    pub last_update_time: u64,
}

impl PriceStorage {
    pub fn new() -> Self {
        Self {
            history: HashMap::new(),
            archives: HashMap::new(),
            ohlc_builders: HashMap::new(),
            policy: Policy {
                aggregation: "LAST".to_string(),
                retain_history: 30,
            },
            version: 0,
            total_updates: 0,
            last_update_time: 0,
        }
    }

    pub fn add_price_with_history(&mut self, symbol: Symbol, price: Price) {
        PRICES.with(|prices| {
            prices.borrow_mut().insert(symbol.clone(), price.clone());
        });

        let price_clone = price.clone();
        self.history
            .entry(symbol.clone())
            .or_insert_with(|| RingBuffer::new(HISTORY_CAPACITY))
            .push(price_clone);

        self.update_ohlc_and_archive(symbol, price);
    }

    fn update_ohlc_and_archive(&mut self, symbol: Symbol, price: Price) {
        self.archives.entry(symbol.clone()).or_insert_with(Archive::new);
        
        let symbol_builders = self.ohlc_builders.entry(symbol.clone()).or_insert_with(HashMap::new);
        
        let resolutions = vec![("1m", 60), ("5m", 300), ("1h", 3600)];
        
        for (resolution_name, resolution_seconds) in resolutions {
            let period_start = (price.timestamp / (resolution_seconds * 1_000_000_000)) * (resolution_seconds * 1_000_000_000);
            
            let builder = symbol_builders.entry(resolution_name.to_string())
                .or_insert_with(|| OHLCBuilder::new(period_start));
            
            if builder.get_start_time() != period_start {
                if let Some(completed_bar) = builder.build_clone() {
                    // Add completed bar to archive
                    if let Some(archive) = self.archives.get_mut(&symbol) {
                        archive.add_bar_to_tier(resolution_name, completed_bar);
                    }
                }
                *builder = OHLCBuilder::new(period_start);
            }
            
            // Add price to current builder
            builder.add_price(&price);
        }
    }

    pub fn get_price(&self, symbol: &Symbol) -> Option<Price> {
        PRICES.with(|prices| {
            prices.borrow().get(symbol)
        })
    }

    pub fn get_prices(&self, symbols: &[Symbol]) -> Vec<Option<Price>> {
        PRICES.with(|prices| {
            let prices_ref = prices.borrow();
            symbols.iter()
                .map(|symbol| prices_ref.get(symbol))
                .collect()
        })
    }

    pub fn get_all_symbols(&self) -> Vec<Symbol> {
        PRICES.with(|prices| {
            prices.borrow().iter().map(|(k, _)| k).collect()
        })
    }

    pub fn get_all_prices(&self) -> Vec<(Symbol, Price)> {
        PRICES.with(|prices| {
            prices.borrow().iter().collect()
        })
    }

    pub fn get_bars(&self, symbol: &Symbol, resolution: &str, start: u64, end: u64) -> Vec<Bar> {
        self.archives.get(symbol)
            .map(|archive| archive.get_bars(resolution, start, end))
            .unwrap_or_default()
    }

    pub fn get_history(&self, symbol: &Symbol) -> Vec<Price> {
        self.history.get(symbol)
            .map(|ring_buffer| ring_buffer.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn get_history_count(&self, symbol: &Symbol) -> usize {
        self.history.get(symbol)
            .map(|ring_buffer| ring_buffer.len())
            .unwrap_or(0)
    }

    pub fn remove_symbol(&mut self, symbol: &Symbol) {
        PRICES.with(|prices| {
            prices.borrow_mut().remove(symbol);
        });
        SYMBOLS.with(|symbols| {
            symbols.borrow_mut().remove(symbol);
        });
        self.history.remove(symbol);
        self.archives.remove(symbol);
        self.ohlc_builders.remove(symbol);
    }

    pub fn upsert_symbol(&self, symbol: &Symbol) {
        SYMBOLS.with(|symbols| {
            symbols.borrow_mut().insert(symbol.clone(), ());
        });
    }

    pub fn symbol_exists(&self, symbol: &Symbol) -> bool {
        SYMBOLS.with(|symbols| {
            symbols.borrow().contains_key(symbol)
        })
    }

    pub fn symbols_is_empty(&self) -> bool {
        SYMBOLS.with(|symbols| {
            symbols.borrow().is_empty()
        })
    }

    pub fn add_manager(&self, principal: &Principal) {
        MANAGERS.with(|managers| {
            managers.borrow_mut().insert(principal.to_text(), ());
        });
    }

    pub fn is_manager(&self, principal: &Principal) -> bool {
        MANAGERS.with(|managers| {
            managers.borrow().contains_key(&principal.to_text())
        })
    }

    pub fn set_managers(&self, principals: Vec<Principal>) {
        MANAGERS.with(|managers| {
            // Remove all existing managers first
            let keys_to_remove: Vec<String> = managers.borrow().iter().map(|(k, _)| k).collect();
            for key in keys_to_remove {
                managers.borrow_mut().remove(&key);
            }
            // Add new managers
            for principal in principals {
                managers.borrow_mut().insert(principal.to_text(), ());
            }
        });
    }

    pub fn is_updater(&self, principal: &Principal) -> bool {
        UPDATERS.with(|updaters| {
            updaters.borrow().contains_key(&principal.to_text())
        })
    }

    pub fn updaters_is_empty(&self) -> bool {
        UPDATERS.with(|updaters| {
            updaters.borrow().is_empty()
        })
    }

    pub fn set_updaters(&self, principals: Vec<Principal>) {
        UPDATERS.with(|updaters| {
            let keys_to_remove: Vec<String> = updaters.borrow().iter().map(|(k, _)| k).collect();
            for key in keys_to_remove {
                updaters.borrow_mut().remove(&key);
            }
            for principal in principals {
                updaters.borrow_mut().insert(principal.to_text(), ());
            }
        });
    }
}

pub fn with_storage<R>(f: impl FnOnce(&PriceStorage) -> R) -> R {
    PRICE_STORAGE.with(|storage| f(&storage.borrow()))
}

pub fn with_storage_mut<R>(f: impl FnOnce(&mut PriceStorage) -> R) -> R {
    PRICE_STORAGE.with(|storage| f(&mut storage.borrow_mut()))
}