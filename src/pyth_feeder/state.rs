use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell};
use std::cell::RefCell;
use crate::types::{PythFeed, FeederConfig, FeederMetrics};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const CONFIG_MEMORY_ID: MemoryId = MemoryId::new(0);
const FEEDS_MEMORY_ID: MemoryId = MemoryId::new(1);
const METRICS_MEMORY_ID: MemoryId = MemoryId::new(2);
const MANAGER_MEMORY_ID: MemoryId = MemoryId::new(3);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static CONFIG: RefCell<StableCell<FeederConfig, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(CONFIG_MEMORY_ID)),
            FeederConfig {
                oracle_canister_id: Principal::anonymous(),
                sol_rpc_canister_id: Principal::anonymous(),
                update_interval_secs: 10,
            }
        ).expect("Failed to initialize CONFIG")
    );

    static FEEDS: RefCell<StableBTreeMap<String, PythFeed, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(FEEDS_MEMORY_ID))
        )
    );

    static METRICS: RefCell<StableCell<FeederMetrics, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(METRICS_MEMORY_ID)),
            FeederMetrics {
                total_updates: 0,
                successful_updates: 0,
                failed_updates: 0,
                last_update_time: 0,
                active_feeds: 0,
                timer_running: false,
            }
        ).expect("Failed to initialize METRICS")
    );

    static MANAGER: RefCell<StableCell<Principal, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MANAGER_MEMORY_ID)),
            Principal::anonymous()
        ).expect("Failed to initialize MANAGER")
    );
}

pub fn get_config() -> FeederConfig {
    CONFIG.with(|c| c.borrow().get().clone())
}

pub fn set_config(config: FeederConfig) {
    CONFIG.with(|c| {
        c.borrow_mut().set(config).expect("Failed to set config");
    });
}

pub fn add_feed(feed: PythFeed) {
    FEEDS.with(|f| {
        f.borrow_mut().insert(feed.symbol.clone(), feed);
    });
}

pub fn remove_feed(symbol: &str) -> Option<PythFeed> {
    FEEDS.with(|f| f.borrow_mut().remove(&symbol.to_string()))
}

pub fn get_feed(symbol: &str) -> Option<PythFeed> {
    FEEDS.with(|f| f.borrow().get(&symbol.to_string()))
}

pub fn get_all_feeds() -> Vec<PythFeed> {
    FEEDS.with(|f| {
        f.borrow()
            .iter()
            .map(|(_, feed)| feed)
            .collect()
    })
}

pub fn get_enabled_feeds() -> Vec<PythFeed> {
    FEEDS.with(|f| {
        f.borrow()
            .iter()
            .map(|(_, feed)| feed)
            .filter(|feed| feed.enabled)
            .collect()
    })
}

pub fn get_metrics() -> FeederMetrics {
    METRICS.with(|m| m.borrow().get().clone())
}

pub fn update_metrics<F>(updater: F)
where
    F: FnOnce(&mut FeederMetrics),
{
    METRICS.with(|m| {
        let mut metrics = m.borrow().get().clone();
        updater(&mut metrics);
        m.borrow_mut().set(metrics).expect("Failed to update metrics");
    });
}

pub fn get_manager() -> Principal {
    MANAGER.with(|m| m.borrow().get().clone())
}

pub fn set_manager(manager: Principal) {
    MANAGER.with(|m| {
        m.borrow_mut().set(manager).expect("Failed to set manager");
    });
}

pub fn is_manager(principal: &Principal) -> bool {
    let manager = get_manager();
    manager == *principal
}
