# XFusion Oracle Canister

A high-performance, decentralized price oracle running on the Internet Computer Protocol (ICP) blockchain. Provides real-time and historical price data for crypto assets and equities with certified data integrity.

## Features

### Core Functionality
- **Real-time Price Updates**: Sub-second latency for price queries
- **Multi-Asset Support**: Handles ~30 crypto assets and equity symbols
- **Historical Data**: Tiered archive system with configurable retention
- **Certified Queries**: Merkle tree-based data certification for trustless verification
- **Role-Based Access**: Separate manager and updater roles for security

### Technical Highlights
- **High Throughput**: Batch price updates every 15 seconds
- **Memory Efficient**: Ring buffer for recent data, segmented archives for historical
- **Data Validation**: Comprehensive input validation and staleness checks
- **OHLC Aggregation**: Automatic candlestick data generation at multiple resolutions
- **Operational Metrics**: Built-in monitoring and health endpoints

## Architecture

```
┌─────────────────┐     ┌──────────────┐
│  Updater App    │────▶│    Oracle    │
│  (Off-chain)    │     │   Canister   │
└─────────────────┘     └──────────────┘
                               │
                               ▼
                    ┌────────────────────┐
                    │   Storage Tiers    │
                    ├────────────────────┤
                    │ Hot:  Ring Buffer  │
                    │ Warm: 1m OHLC      │
                    │ Cold: 5m/1h OHLC   │
                    └────────────────────┘
```

## API Reference

### Query Methods

#### `get_price(symbol: Symbol) -> Option<Price>`
Returns the latest price for a specific symbol.

#### `get_prices(symbols: Vec<Symbol>) -> Vec<Option<Price>>`
Batch query for multiple symbol prices.

#### `get_all_symbols() -> Vec<Symbol>`
Returns all currently tracked symbols.

#### `get_snapshot_cert() -> (Vec<(Symbol, Price)>, ByteBuf)`
Returns certified snapshot of all current prices with Merkle root.

#### `get_range(symbol: Symbol, start: u64, end: u64, resolution: String) -> Vec<Bar>`
Retrieves historical OHLC data for specified time range and resolution.

#### `get_metrics() -> OracleMetrics`
Returns operational metrics including update counts, cycles balance, and version.

### Update Methods

#### `push_prices(updates: Vec<PriceUpdate>) -> u64`
Updates prices for multiple symbols (requires updater authorization).

### Admin Methods

#### `set_allowed_updaters(principals: Vec<Principal>)`
Configures authorized price updaters (manager only).

#### `set_managers(principals: Vec<Principal>)`
Updates manager list (manager only).

#### `upsert_symbols(symbols: Vec<Symbol>)`
Adds symbols to the allowed list (manager only).

#### `remove_symbols(symbols: Vec<Symbol>)`
Removes symbols and their data (manager only).

#### `set_policy(policy: Policy)`
Updates retention and aggregation policy (manager only).

## Data Types

```rust
type Symbol = String;

type Price = {
    value: u64;           // Price in smallest unit
    confidence: ?u64;     // Optional confidence interval
    timestamp: u64;       // Unix timestamp in nanoseconds
    source: String;       // Data source identifier
};

type Bar = {
    timestamp: u64;       // Period start time
    open: u64;
    high: u64;
    low: u64;
    close: u64;
    volume: ?u64;         // Optional volume data
};
```

## Installation

### Prerequisites
- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install) (>= 0.15.0)
- Rust toolchain with `wasm32-unknown-unknown` target
- Node.js (>= 16) for frontend integration

### Local Development

1. Clone the repository:
```bash
git clone https://github.com/your-org/xfusion-oracle-canister
cd xfusion-oracle-canister
```

2. Start local replica:
```bash
dfx start --clean
```

3. Deploy the canister:
```bash
dfx deploy oracle
```

4. Initialize with test data:
```bash
dfx canister call oracle upsert_symbols '(vec {"BTC"; "ETH"; "SOL"})'
```

### Mainnet Deployment

1. Configure mainnet network:
```bash
dfx identity use production
dfx wallet --network ic balance
```

2. Deploy to mainnet:
```bash
dfx deploy --network ic oracle
```

3. Set initial managers:
```bash
dfx canister --network ic call oracle set_managers '(vec {principal "your-principal-id"})'
```

## Security Considerations

### Access Control
- **Managers**: Can modify updater list, symbol registry, and policies
- **Updaters**: Can only push price updates
- **Public**: Read-only access to all price data

### Data Validation
- Timestamps must be within ±5 minutes of current time
- Price values must be non-zero and within reasonable bounds
- Confidence intervals cannot exceed price values
- Symbol must be in allowed list (if registry enabled)

### Best Practices
1. Use multiple independent updater principals
2. Implement rate limiting in updater application
3. Monitor metrics regularly for anomalies
4. Rotate updater credentials periodically
5. Backup historical data off-chain

## Performance

### Capacity
- **Symbols**: ~50 concurrent assets
- **History**: 30 days at 15-second intervals (per symbol)
- **Archives**: 1+ year of 1-minute OHLC data

### Costs (Estimated)
- **Storage**: $17-64/year depending on subnet
- **Updates**: ~$2-8/month for 15-second updates
- **Queries**: ~$16/month per 1M queries

### Optimization Tips
- Batch price updates to minimize cycles
- Use `get_prices` for multiple symbols
- Cache frequently accessed data client-side
- Configure appropriate retention policies

## Development

### Building from Source
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Running Tests
```bash
cargo test
```

### Code Structure
```
src/
├── lib.rs          # Canister endpoints
├── types.rs        # Data structures
├── state.rs        # Storage management
├── ring_buffer.rs  # Circular buffer implementation
├── archive.rs      # Tiered storage system
├── ohlc.rs         # OHLC aggregation logic
└── merkle.rs       # Certification logic
```

## Monitoring

Track canister health using the metrics endpoint:

```bash
dfx canister call oracle get_metrics
```

Key metrics to monitor:
- `total_updates`: Should increase regularly
- `last_update_time`: Should be recent
- `canister_cycles`: Must stay above minimum threshold
- `total_symbols`: Active symbol count
 