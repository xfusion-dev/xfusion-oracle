use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::Bar;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArchiveSegment {
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub bars: Vec<Bar>,
}

impl ArchiveSegment {
    pub fn new(start_timestamp: u64) -> Self {
        Self {
            start_timestamp,
            end_timestamp: start_timestamp,
            bars: Vec::new(),
        }
    }

    pub fn add_bar(&mut self, bar: Bar) {
        self.end_timestamp = bar.timestamp;
        self.bars.push(bar);
    }

    pub fn is_in_range(&self, start: u64, end: u64) -> bool {
        self.start_timestamp <= end && self.end_timestamp >= start
    }

    pub fn get_bars_in_range(&self, start: u64, end: u64) -> Vec<&Bar> {
        self.bars
            .iter()
            .filter(|bar| bar.timestamp >= start && bar.timestamp <= end)
            .collect()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArchiveTier {
    pub resolution: u64, // seconds per bar
    pub segments: Vec<ArchiveSegment>,
}

impl ArchiveTier {
    pub fn new(resolution: u64) -> Self {
        Self {
            resolution,
            segments: Vec::new(),
        }
    }

    pub fn add_bar(&mut self, bar: Bar) {
        let segment_start = (bar.timestamp / (24 * 60 * 60)) * (24 * 60 * 60); // daily segments

        if let Some(last_segment) = self.segments.last_mut() {
            if last_segment.start_timestamp == segment_start {
                last_segment.add_bar(bar);
                return;
            }
        }

        let mut new_segment = ArchiveSegment::new(segment_start);
        new_segment.add_bar(bar);
        self.segments.push(new_segment);
    }

    pub fn get_bars_in_range(&self, start: u64, end: u64) -> Vec<Bar> {
        let mut result = Vec::new();

        for segment in &self.segments {
            if segment.is_in_range(start, end) {
                for bar in segment.get_bars_in_range(start, end) {
                    result.push(bar.clone());
                }
            }
        }

        result.sort_by_key(|bar| bar.timestamp);
        result
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Archive {
    pub tiers: HashMap<String, ArchiveTier>, // tier name -> tier
}

impl Archive {
    pub fn new() -> Self {
        let mut archive = Self {
            tiers: HashMap::new(),
        };

        // Initialize standard tiers
        archive.tiers.insert("1m".to_string(), ArchiveTier::new(60));       // 1 minute
        archive.tiers.insert("5m".to_string(), ArchiveTier::new(300));      // 5 minutes
        archive.tiers.insert("1h".to_string(), ArchiveTier::new(3600));     // 1 hour

        archive
    }

    pub fn add_bar_to_tier(&mut self, tier_name: &str, bar: Bar) {
        if let Some(tier) = self.tiers.get_mut(tier_name) {
            tier.add_bar(bar);
        }
    }

    pub fn get_bars(&self, tier_name: &str, start: u64, end: u64) -> Vec<Bar> {
        self.tiers
            .get(tier_name)
            .map(|tier| tier.get_bars_in_range(start, end))
            .unwrap_or_default()
    }
}