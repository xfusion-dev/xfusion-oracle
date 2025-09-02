use ic_cdk_macros::*;

mod types;
mod state;
mod ring_buffer;
mod ohlc;

use types::{Symbol, Price, Bar, Policy, PriceUpdate};
use state::{with_storage, with_storage_mut};

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

#[update]
fn push_prices(updates: Vec<PriceUpdate>) -> u64 {
    let caller = ic_cdk::caller();

    with_storage_mut(|storage| {
        if !storage.allowed_updaters.is_empty() && !storage.allowed_updaters.contains(&caller) {
            ic_cdk::trap("Unauthorized");
        }

        let current_time = ic_cdk::api::time();

        for update in updates {
            if update.price.value == 0 {
                continue;
            }

            if update.price.timestamp > current_time + 60_000_000_000 {
                continue;
            }

            storage.add_price_with_history(update.symbol, update.price);
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