use ic_cdk_macros::*;

mod types;
mod state;

use types::{Symbol, Price, Bar, Policy, PriceUpdate};
use state::{with_storage, with_storage_mut};

#[init]
fn init() {
    ic_cdk::println!("Oracle canister initialized");
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