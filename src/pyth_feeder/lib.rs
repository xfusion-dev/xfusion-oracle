mod pyth;
mod state;
mod types;

use candid::{CandidType, Principal};
use ic_cdk_macros::{init, post_upgrade, query, update};
use ic_cdk_timers::TimerId;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::pyth::PriceAccount;
use crate::state::*;
use crate::types::*;

static mut TIMER_ID: Option<TimerId> = None;

#[derive(CandidType, Serialize, Deserialize)]
struct OraclePriceUpdate {
    symbol: String,
    price: i64,
    confidence: Option<u64>,
    timestamp: u64,
    source: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
enum RpcSources {
    #[serde(rename = "Default")]
    Default(SolanaCluster),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
enum SolanaCluster {
    Mainnet,
    Devnet,
    Testnet,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
enum GetAccountInfoEncoding {
    #[serde(rename = "base58")]
    base58,
    #[serde(rename = "base64")]
    base64,
    #[serde(rename = "base64+zstd")]
    base64zstd,
    #[serde(rename = "jsonParsed")]
    jsonParsed,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
struct GetAccountInfoParams {
    pub pubkey: String,
    pub commitment: Option<String>,
    pub encoding: Option<GetAccountInfoEncoding>,
    pub dataSlice: Option<()>,
    pub minContextSlot: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
enum MultiGetAccountInfoResult {
    Consistent(GetAccountInfoResult),
    Inconsistent(Vec<((), GetAccountInfoResult)>),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
enum GetAccountInfoResult {
    Ok(Option<AccountInfo>),
    Err(()),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
struct AccountInfo {
    pub lamports: u64,
    pub data: AccountData,
    pub owner: String,
    pub executable: bool,
    pub rentEpoch: u64,
    pub space: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
enum AccountData {
    #[serde(rename = "legacyBinary")]
    legacyBinary(String),
    #[serde(rename = "binary")]
    binary((String, AccountEncoding)),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
enum AccountEncoding {
    binary,
    base58,
    base64,
    #[serde(rename = "base64+zstd")]
    base64zstd,
    jsonParsed,
}

#[init]
fn init(args: InitArgs) {
    let config = FeederConfig {
        oracle_canister_id: args.oracle_canister_id.unwrap_or(Principal::anonymous()),
        sol_rpc_canister_id: args.sol_rpc_canister_id,
        update_interval_secs: args.update_interval_secs.unwrap_or(10),
    };
    set_config(config);
    ic_cdk::println!("🚀 Pyth Feeder initialized (Testing mode - logging only)");
}

#[post_upgrade]
fn post_upgrade() {
    let config = get_config();
    if config.oracle_canister_id != Principal::anonymous() {
        start_timer_internal();
    }
}

#[update]
fn start_timer() {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        ic_cdk::trap("Anonymous principal cannot start timer");
    }

    start_timer_internal();
}

fn start_timer_internal() {
    unsafe {
        if TIMER_ID.is_some() {
            return;
        }
    }

    let config = get_config();
    let interval = Duration::from_secs(config.update_interval_secs);

    let timer_id = ic_cdk_timers::set_timer_interval(interval, || {
        ic_cdk::spawn(async {
            let _ = fetch_and_push_prices().await;
        });
    });

    unsafe {
        TIMER_ID = Some(timer_id);
    }

    update_metrics(|m| m.timer_running = true);
}

#[update]
fn stop_timer() {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        ic_cdk::trap("Anonymous principal cannot stop timer");
    }

    unsafe {
        if let Some(timer_id) = TIMER_ID {
            ic_cdk_timers::clear_timer(timer_id);
            TIMER_ID = None;
        }
    }

    update_metrics(|m| m.timer_running = false);
}

async fn fetch_and_push_prices() -> Result<Vec<FeedUpdateResult>, String> {
    let config = get_config();
    let feeds = get_enabled_feeds();

    if feeds.is_empty() {
        return Ok(vec![]);
    }

    ic_cdk::println!("🔄 Starting price fetch for {} feeds...", feeds.len());

    let mut results = Vec::new();
    let mut oracle_updates = Vec::new();

    for feed in feeds {
        match fetch_pyth_price(&config, &feed).await {
            Ok((price, confidence)) => {
                let timestamp = ic_cdk::api::time();

                ic_cdk::println!(
                    "✓ {} | Price: ${:.2} | Confidence: ±${:.2} | Timestamp: {}",
                    feed.symbol,
                    price,
                    confidence,
                    timestamp
                );

                oracle_updates.push(OraclePriceUpdate {
                    symbol: feed.symbol.clone(),
                    price: (price * 1_000_000.0) as i64,
                    confidence: Some((confidence * 1_000_000.0) as u64),
                    timestamp,
                    source: "pyth".to_string(),
                });

                results.push(FeedUpdateResult {
                    symbol: feed.symbol.clone(),
                    success: true,
                    error: None,
                    price: Some(price),
                    confidence: Some(confidence),
                });

                update_metrics(|m| m.successful_updates += 1);
            }
            Err(e) => {
                ic_cdk::println!(
                    "✗ {} | Error: {}",
                    feed.symbol,
                    e
                );

                results.push(FeedUpdateResult {
                    symbol: feed.symbol.clone(),
                    success: false,
                    error: Some(e),
                    price: None,
                    confidence: None,
                });

                update_metrics(|m| m.failed_updates += 1);
            }
        }
    }

    if !oracle_updates.is_empty() {
        ic_cdk::println!("📊 Fetched {} prices successfully (Oracle push disabled for testing)", oracle_updates.len());
    }

    update_metrics(|m| {
        m.total_updates += 1;
        m.last_update_time = ic_cdk::api::time();
    });

    Ok(results)
}

async fn fetch_pyth_price(config: &FeederConfig, feed: &PythFeed) -> Result<(f64, f64), String> {
    let params = GetAccountInfoParams {
        pubkey: feed.account.clone(),
        commitment: None,
        encoding: Some(GetAccountInfoEncoding::base64),
        dataSlice: None,
        minContextSlot: None,
    };

    let sources = RpcSources::Default(SolanaCluster::Mainnet);
    let rpc_config: Option<()> = None;

    let cycles = 100_000_000_000u128;

    let call_result: Result<(MultiGetAccountInfoResult,), _> = ic_cdk::api::call::call_with_payment128(
        config.sol_rpc_canister_id,
        "getAccountInfo",
        (sources, rpc_config, params),
        cycles
    ).await;

    match call_result {
        Ok((MultiGetAccountInfoResult::Consistent(GetAccountInfoResult::Ok(Some(account_info))),)) => {
            let bytes = match account_info.data {
                AccountData::binary((base64_data, _)) => {
                    use base64::Engine;
                    base64::engine::general_purpose::STANDARD.decode(&base64_data)
                        .map_err(|e| format!("Failed to decode base64: {}", e))?
                }
                AccountData::legacyBinary(base64_data) => {
                    use base64::Engine;
                    base64::engine::general_purpose::STANDARD.decode(&base64_data)
                        .map_err(|e| format!("Failed to decode base64: {}", e))?
                }
            };

            ic_cdk::println!(
                "DEBUG {} | Account data length: {} bytes | First 50 bytes: {:?}",
                feed.symbol,
                bytes.len(),
                &bytes[..bytes.len().min(50)]
            );

            if bytes.len() == 134 {
                if bytes.len() < 117 {
                    return Err("Account too small for new Pyth format".to_string());
                }

                let price_bytes = &bytes[109..117];
                let raw_price = i64::from_le_bytes([
                    price_bytes[0], price_bytes[1], price_bytes[2], price_bytes[3],
                    price_bytes[4], price_bytes[5], price_bytes[6], price_bytes[7],
                ]);

                let price = (raw_price as f64) * 1e-8;
                let confidence = 0.0;

                ic_cdk::println!(
                    "DEBUG {} | New format | Raw price: {} | Price: ${:.2}",
                    feed.symbol,
                    raw_price,
                    price
                );

                if raw_price == 0 {
                    return Err("Price is zero".to_string());
                }

                Ok((price, confidence))
            } else {
                let price_account = PriceAccount::parse(&bytes)?;

                ic_cdk::println!(
                    "DEBUG {} | Legacy format | Status: {:?} | Raw price: {} | Exponent: {}",
                    feed.symbol,
                    price_account.agg.status,
                    price_account.agg.price,
                    price_account.exponent
                );

                let price = price_account.get_price_as_f64();
                let confidence = price_account.get_confidence_as_f64();

                if !price_account.is_valid() {
                    return Err(format!("Price not valid (status: {:?}, price: {})", price_account.agg.status, price));
                }

                Ok((price, confidence))
            }
        }
        Ok((MultiGetAccountInfoResult::Consistent(GetAccountInfoResult::Ok(None)),)) => {
            Err("Account not found".to_string())
        }
        Ok((MultiGetAccountInfoResult::Consistent(GetAccountInfoResult::Err(_)),)) => {
            Err("RPC returned error".to_string())
        }
        Ok((MultiGetAccountInfoResult::Inconsistent(_),)) => {
            Err("RPC returned inconsistent results".to_string())
        }
        Err(e) => {
            Err(format!("RPC call failed: {:?}", e))
        }
    }
}

async fn push_to_oracle(config: &FeederConfig, updates: Vec<OraclePriceUpdate>) -> Result<(), String> {
    let call_result: Result<(), _> = ic_cdk::call(
        config.oracle_canister_id,
        "push_prices",
        (updates,)
    ).await;

    call_result.map_err(|e| format!("Oracle call failed: {:?}", e))
}

#[update]
fn add_pyth_feed(symbol: String, account: String, enabled: bool) {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        ic_cdk::trap("Anonymous principal cannot add feeds");
    }

    let feed = PythFeed {
        symbol,
        account,
        enabled,
    };

    add_feed(feed);

    let active_feeds = get_enabled_feeds().len() as u32;
    update_metrics(|m| m.active_feeds = active_feeds);
}

#[update]
fn remove_pyth_feed(symbol: String) -> bool {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        ic_cdk::trap("Anonymous principal cannot remove feeds");
    }

    let result = remove_feed(&symbol).is_some();

    let active_feeds = get_enabled_feeds().len() as u32;
    update_metrics(|m| m.active_feeds = active_feeds);

    result
}

#[update]
fn toggle_feed(symbol: String, enabled: bool) -> bool {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        ic_cdk::trap("Anonymous principal cannot toggle feeds");
    }

    if let Some(mut feed) = get_feed(&symbol) {
        feed.enabled = enabled;
        add_feed(feed);

        let active_feeds = get_enabled_feeds().len() as u32;
        update_metrics(|m| m.active_feeds = active_feeds);

        true
    } else {
        false
    }
}

#[query]
fn get_pyth_feeds() -> Vec<PythFeed> {
    get_all_feeds()
}

#[query]
fn get_pyth_feed(symbol: String) -> Option<PythFeed> {
    get_feed(&symbol)
}

#[query]
fn get_feeder_metrics() -> FeederMetrics {
    get_metrics()
}

#[query]
fn get_feeder_config() -> FeederConfig {
    get_config()
}

#[update]
fn set_feeder_config(config: FeederConfig) {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        ic_cdk::trap("Anonymous principal cannot set config");
    }

    set_config(config);
}

#[update]
async fn manual_fetch() -> Vec<FeedUpdateResult> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        ic_cdk::trap("Anonymous principal cannot trigger manual fetch");
    }

    fetch_and_push_prices()
        .await
        .unwrap_or_else(|e| vec![FeedUpdateResult {
            symbol: "error".to_string(),
            success: false,
            error: Some(e),
            price: None,
            confidence: None,
        }])
}

ic_cdk::export_candid!();
