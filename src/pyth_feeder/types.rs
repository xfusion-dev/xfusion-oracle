use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct PythFeed {
    pub symbol: String,
    pub account: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct FeedUpdateResult {
    pub symbol: String,
    pub success: bool,
    pub error: Option<String>,
    pub price: Option<f64>,
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct FeederMetrics {
    pub total_updates: u64,
    pub successful_updates: u64,
    pub failed_updates: u64,
    pub last_update_time: u64,
    pub active_feeds: u32,
    pub timer_running: bool,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct FeederConfig {
    pub oracle_canister_id: Principal,
    pub sol_rpc_canister_id: Principal,
    pub update_interval_secs: u64,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct InitArgs {
    pub oracle_canister_id: Option<Principal>,
    pub sol_rpc_canister_id: Principal,
    pub update_interval_secs: Option<u64>,
}

impl Storable for PythFeed {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for FeederMetrics {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 200,
        is_fixed_size: false,
    };
}

impl Storable for FeederConfig {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 200,
        is_fixed_size: false,
    };
}
