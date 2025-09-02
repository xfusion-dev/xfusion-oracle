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
    with_storage_mut(|storage| {
        storage.managers.insert(deployer);
    });
    ic_cdk::println!("Oracle canister initialized with deployer as manager");
}

#[query]
fn get_price(symbol: Symbol) -> Option<Price> {
    with_storage(|storage| {
        storage.prices.get(&symbol).cloned()
    })
}

#[query]
fn get_prices(symbols: Vec<Symbol>) -> Vec<Option<Price>> {
    with_storage(|storage| {
        symbols.iter()
            .map(|symbol| storage.prices.get(symbol).cloned())
            .collect()
    })
}

#[query]
fn get_all_symbols() -> Vec<Symbol> {
    with_storage(|storage| {
        storage.prices.keys().cloned().collect()
    })
}

#[query]
fn get_range(symbol: Symbol, start: u64, end: u64, resolution: String) -> Vec<Bar> {
    with_storage(|storage| {
        if let Some(archive) = storage.archives.get(&symbol) {
            archive.get_bars(&resolution, start, end)
        } else {
            vec![]
        }
    })
}

#[query]
fn get_snapshot_cert() -> (Vec<(Symbol, Price)>, ByteBuf) {
    with_storage(|storage| {
        let snapshot: Vec<(Symbol, Price)> = storage
            .prices
            .iter()
            .map(|(symbol, price)| (symbol.clone(), price.clone()))
            .collect();

        create_certified_snapshot(snapshot)
    })
}

#[update]
fn push_prices(updates: Vec<PriceUpdate>) -> u64 {
    let caller = ic_cdk::caller();

    with_storage_mut(|storage| {
        if !storage.allowed_updaters.is_empty() && !storage.allowed_updaters.contains(&caller) {
            ic_cdk::trap("Unauthorized");
        }

        let current_time = ic_cdk::api::time();
        let max_future_time = current_time + 60_000_000_000; // 1 minute in nanoseconds
        let min_past_time = current_time - 300_000_000_000; // 5 minutes in nanoseconds

        for update in updates {
            // Validate price value
            if update.price.value == 0 {
                continue;
            }

            // Validate price is not absurdly high or low
            if update.price.value > 1_000_000_000_000 {
                continue;
            }

            // Validate timestamp is not too far in future
            if update.price.timestamp > max_future_time {
                continue;
            }

            // Validate timestamp is not too stale
            if update.price.timestamp < min_past_time {
                continue;
            }

            // Validate symbol is allowed (if symbol registry is used)
            if !storage.symbols.is_empty() && !storage.symbols.contains(&update.symbol) {
                continue;
            }

            // Validate confidence if provided
            if let Some(conf) = update.price.confidence {
                if conf > update.price.value {
                    continue;
                }
            }

            // Validate source is not empty
            if update.price.source.is_empty() {
                continue;
            }

            storage.add_price_with_history(update.symbol, update.price);
            storage.total_updates += 1;
            storage.last_update_time = current_time;
        }

        storage.version += 1;
        storage.version
    })
}

#[update]
fn set_allowed_updaters(principals: Vec<candid::Principal>) {
    let caller = ic_cdk::caller();

    with_storage_mut(|storage| {
        if !storage.managers.contains(&caller) {
            ic_cdk::trap("Unauthorized: only managers can modify updaters");
        }
        storage.allowed_updaters = principals.into_iter().collect();
    })
}

#[update]
fn set_managers(principals: Vec<candid::Principal>) {
    let caller = ic_cdk::caller();

    with_storage_mut(|storage| {
        if !storage.managers.contains(&caller) {
            ic_cdk::trap("Unauthorized: only managers can modify managers");
        }
        storage.managers = principals.into_iter().collect();
    })
}

#[update]
fn upsert_symbols(symbols: Vec<Symbol>) {
    let caller = ic_cdk::caller();

    with_storage_mut(|storage| {
        if !storage.managers.contains(&caller) {
            ic_cdk::trap("Unauthorized: only managers can modify symbols");
        }
        for symbol in symbols {
            storage.symbols.insert(symbol);
        }
    })
}

#[update]
fn remove_symbols(symbols: Vec<Symbol>) {
    let caller = ic_cdk::caller();

    with_storage_mut(|storage| {
        if !storage.managers.contains(&caller) {
            ic_cdk::trap("Unauthorized: only managers can modify symbols");
        }
        for symbol in symbols {
            storage.symbols.remove(&symbol);
            storage.prices.remove(&symbol);
            storage.history.remove(&symbol);
            storage.archives.remove(&symbol);
        }
    })
}

#[update]
fn set_policy(new_policy: Policy) {
    let caller = ic_cdk::caller();

    with_storage_mut(|storage| {
        if !storage.managers.contains(&caller) {
            ic_cdk::trap("Unauthorized: only managers can modify policy");
        }
        storage.policy = new_policy;
    })
}

#[query]
fn get_metrics() -> OracleMetrics {
    with_storage(|storage| {
        OracleMetrics {
            total_symbols: storage.prices.len() as u64,
            total_updates: storage.total_updates,
            last_update_time: storage.last_update_time,
            canister_cycles: ic_cdk::api::canister_balance(),
            version: storage.version,
        }
    })
}