use ic_cdk_macros::*;

#[init]
fn init() {
    ic_cdk::println!("Oracle canister initialized");
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}