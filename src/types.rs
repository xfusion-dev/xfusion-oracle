use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

pub type Symbol = String;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Price {
    pub value: u64,
    pub confidence: Option<u64>,
    pub timestamp: u64,
    pub source: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Bar {
    pub timestamp: u64,
    pub open: u64,
    pub high: u64,
    pub low: u64,
    pub close: u64,
    pub volume: Option<u64>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Policy {
    pub aggregation: String,
    pub retain_history: u32,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PriceUpdate {
    pub symbol: Symbol,
    pub price: Price,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct OracleMetrics {
    pub total_symbols: u64,
    pub total_updates: u64,
    pub last_update_time: u64,
    pub canister_cycles: u64,
    pub version: u64,
}