use ic_cdk_macros::*;

mod types;

use types::{Symbol, Price, Bar, Policy, PriceUpdate};

#[init]
fn init() {
    ic_cdk::println!("Oracle canister initialized");
}

#[query]
fn get_price(symbol: Symbol) -> Option<Price> {
    None
}

#[query]
fn get_prices(symbols: Vec<Symbol>) -> Vec<Option<Price>> {
    symbols.iter().map(|_| None).collect()
}

#[query]
fn get_all_symbols() -> Vec<Symbol> {
    vec![]
}