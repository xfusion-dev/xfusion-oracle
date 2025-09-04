use ic_cdk_macros::*;
use serde_bytes::ByteBuf;

mod types;
mod state;
mod ring_buffer;
mod ohlc;
mod archive;
mod merkle;

use types::{Symbol, Price, Bar, Policy, PriceUpdate, OracleMetrics};
use state::{with_storage, with_storage_mut};
use merkle::create_certified_snapshot;

#[init]
fn init() {
    let deployer = ic_cdk::caller();
    with_storage(|storage| {
        storage.add_manager(&deployer);
    });
    ic_cdk::println!("Oracle canister initialized with deployer as manager");
}

#[query]
fn get_price(symbol: Symbol) -> Option<Price> {
    with_storage(|storage| {
        storage.get_price(&symbol)
    })
}

#[query]
fn get_prices(symbols: Vec<Symbol>) -> Vec<Option<Price>> {
    with_storage(|storage| {
        storage.get_prices(&symbols)
    })
}

#[query]
fn get_all_symbols() -> Vec<Symbol> {
    with_storage(|storage| {
        storage.get_all_symbols()
    })
}

#[query]
fn get_range(symbol: Symbol, start: u64, end: u64, resolution: String) -> Vec<Bar> {
    with_storage(|storage| {
        storage.get_bars(&symbol, &resolution, start, end)
    })
}

#[query]
fn get_price_history(symbol: Symbol) -> Vec<Price> {
    with_storage(|storage| {
        storage.get_history(&symbol)
    })
}

#[query]
fn get_price_history_count(symbol: Symbol) -> u64 {
    with_storage(|storage| {
        storage.get_history_count(&symbol) as u64
    })
}

#[query]
fn get_available_resolutions() -> Vec<String> {
    vec!["1m".to_string(), "5m".to_string(), "1h".to_string()]
}

#[query]
fn get_snapshot_cert() -> (Vec<(Symbol, Price)>, ByteBuf) {
    with_storage(|storage| {
        let snapshot = storage.get_all_prices();
        create_certified_snapshot(snapshot)
    })
}

#[update]
fn push_prices(updates: Vec<PriceUpdate>) -> u64 {
    let caller = ic_cdk::caller();

    // Validate input size to prevent spam attacks
    if updates.len() > 1000 {
        ic_cdk::trap("Too many price updates in single request (max: 1000)");
    }

    with_storage_mut(|storage| {
        if !storage.updaters_is_empty() && !storage.is_updater(&caller) {
            ic_cdk::trap("Unauthorized");
        }

        let current_time = ic_cdk::api::time();
        let max_future_time = current_time + 60_000_000_000; // 1 minute in nanoseconds
        let min_past_time = current_time - 300_000_000_000; // 5 minutes in nanoseconds
        let mut processed_count = 0;

        for update in updates {
            if update.symbol.is_empty() || update.symbol.len() > 50 {
                continue;
            }

            if update.price.value == 0 || update.price.value < 1 {
                continue;
            }

            if update.price.value > 1_000_000_000_000_000 {
                continue;
            }

            if update.price.timestamp > max_future_time {
                continue;
            }

            if update.price.timestamp < min_past_time {
                continue;
            }

            if !storage.symbols_is_empty() && !storage.symbol_exists(&update.symbol) {
                continue;
            }

            if let Some(conf) = update.price.confidence {
                if conf > update.price.value || conf == 0 {
                    continue;
                }
            }

            if update.price.source.is_empty() || update.price.source.len() > 100 {
                continue;
            }

            storage.add_price_with_history(update.symbol, update.price);
            storage.total_updates += 1;
            storage.last_update_time = current_time;
            processed_count += 1;
        }

        if processed_count > 0 {
            storage.version += 1;
        }
        
        storage.version
    })
}

#[update]
fn set_allowed_updaters(principals: Vec<candid::Principal>) {
    let caller = ic_cdk::caller();

    with_storage(|storage| {
        if !storage.is_manager(&caller) {
            ic_cdk::trap("Unauthorized: only managers can modify updaters");
        }
        storage.set_updaters(principals);
    })
}

#[update]
fn set_managers(principals: Vec<candid::Principal>) {
    let caller = ic_cdk::caller();

    with_storage(|storage| {
        if !storage.is_manager(&caller) {
            ic_cdk::trap("Unauthorized: only managers can modify managers");
        }
        storage.set_managers(principals);
    })
}

#[update]
fn upsert_symbols(symbols: Vec<Symbol>) {
    let caller = ic_cdk::caller();

    with_storage(|storage| {
        if !storage.is_manager(&caller) {
            ic_cdk::trap("Unauthorized: only managers can modify symbols");
        }
        for symbol in symbols {
            storage.upsert_symbol(&symbol);
        }
    })
}

#[update]
fn remove_symbols(symbols: Vec<Symbol>) {
    let caller = ic_cdk::caller();

    with_storage_mut(|storage| {
        if !storage.is_manager(&caller) {
            ic_cdk::trap("Unauthorized: only managers can modify symbols");
        }
        for symbol in symbols {
            storage.remove_symbol(&symbol);
        }
    })
}

#[update]
fn set_policy(new_policy: Policy) {
    let caller = ic_cdk::caller();

    with_storage_mut(|storage| {
        if !storage.is_manager(&caller) {
            ic_cdk::trap("Unauthorized: only managers can modify policy");
        }
        storage.policy = new_policy;
    })
}

#[query]
fn get_metrics() -> OracleMetrics {
    with_storage(|storage| {
        let total_symbols = storage.get_all_symbols().len() as u64;
        OracleMetrics {
            total_symbols,
            total_updates: storage.total_updates,
            last_update_time: storage.last_update_time,
            canister_cycles: ic_cdk::api::canister_balance(),
            version: storage.version,
        }
    })
}

#[query]
fn get_managers() -> Vec<String> {
    with_storage(|storage| {
        storage.get_all_managers()
    })
}

#[query]
fn get_updaters() -> Vec<String> {
    with_storage(|storage| {
        storage.get_all_updaters()
    })
}