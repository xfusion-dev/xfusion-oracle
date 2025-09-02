use crate::types::{Price, Bar};

pub struct OHLCBuilder {
    open: Option<u64>,
    high: u64,
    low: u64,
    close: u64,
    start_time: u64,
}

impl OHLCBuilder {
    pub fn new(start_time: u64) -> Self {
        Self {
            open: None,
            high: 0,
            low: u64::MAX,
            close: 0,
            start_time,
        }
    }

    pub fn add_price(&mut self, price: &Price) {
        if self.open.is_none() {
            self.open = Some(price.value);
        }

        self.high = self.high.max(price.value);
        self.low = self.low.min(price.value);
        self.close = price.value;
    }

    pub fn build(self) -> Option<Bar> {
        self.open.map(|open| Bar {
            timestamp: self.start_time,
            open,
            high: self.high,
            low: if self.low == u64::MAX { open } else { self.low },
            close: self.close,
            volume: None,
        })
    }
}

pub fn aggregate_prices_to_bar(prices: &[Price], period_start: u64) -> Option<Bar> {
    if prices.is_empty() {
        return None;
    }

    let mut builder = OHLCBuilder::new(period_start);
    for price in prices {
        builder.add_price(price);
    }
    builder.build()
}