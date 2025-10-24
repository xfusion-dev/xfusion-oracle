use candid::CandidType;
use serde::{Deserialize, Serialize};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountType {
    Unknown = 0,
    Mapping = 1,
    Product = 2,
    Price = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, CandidType, Serialize, Deserialize)]
pub enum PriceStatus {
    Unknown = 0,
    Trading = 1,
    Halted = 2,
    Auction = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceType {
    Unknown = 0,
    Price = 1,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct PriceInfo {
    pub price: i64,
    pub conf: u64,
    pub status: PriceStatus,
    pub corp_act: u32,
    pub pub_slot: u64,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct Ema {
    pub val: i64,
    pub numer: i64,
    pub denom: i64,
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct PriceAccount {
    pub magic: u32,
    pub ver: u32,
    pub atype: u32,
    pub size: u32,
    pub ptype: u32,
    pub exponent: i32,
    pub num: u32,
    pub unused: u32,
    pub curr_slot: u64,
    pub valid_slot: u64,
    pub twap: Ema,
    pub twac: Ema,
    pub drv1: u64,
    pub drv2: PriceInfo,
    pub prod: [u8; 32],
    pub next: [u8; 32],
    pub agg_pub_slot: u64,
    pub agg: PriceInfo,
}

impl PriceAccount {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 208 {
            return Err(format!("Invalid Pyth account data length: {} (expected at least 208)", data.len()));
        }

        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if magic != 0xa1b2c3d4 {
            return Err(format!("Invalid magic number: 0x{:x} (expected 0xa1b2c3d4)", magic));
        }

        let ver = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let atype = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);

        if atype != 3 {
            return Err(format!("Not a price account (type: {})", atype));
        }

        let size = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
        let ptype = u32::from_le_bytes([data[16], data[17], data[18], data[19]]);
        let exponent = i32::from_le_bytes([data[20], data[21], data[22], data[23]]);
        let num = u32::from_le_bytes([data[24], data[25], data[26], data[27]]);
        let unused = u32::from_le_bytes([data[28], data[29], data[30], data[31]]);
        let curr_slot = u64::from_le_bytes([
            data[32], data[33], data[34], data[35],
            data[36], data[37], data[38], data[39],
        ]);
        let valid_slot = u64::from_le_bytes([
            data[40], data[41], data[42], data[43],
            data[44], data[45], data[46], data[47],
        ]);

        let twap = Ema {
            val: i64::from_le_bytes([
                data[48], data[49], data[50], data[51],
                data[52], data[53], data[54], data[55],
            ]),
            numer: i64::from_le_bytes([
                data[56], data[57], data[58], data[59],
                data[60], data[61], data[62], data[63],
            ]),
            denom: i64::from_le_bytes([
                data[64], data[65], data[66], data[67],
                data[68], data[69], data[70], data[71],
            ]),
        };

        let twac = Ema {
            val: i64::from_le_bytes([
                data[72], data[73], data[74], data[75],
                data[76], data[77], data[78], data[79],
            ]),
            numer: i64::from_le_bytes([
                data[80], data[81], data[82], data[83],
                data[84], data[85], data[86], data[87],
            ]),
            denom: i64::from_le_bytes([
                data[88], data[89], data[90], data[91],
                data[92], data[93], data[94], data[95],
            ]),
        };

        let drv1 = u64::from_le_bytes([
            data[96], data[97], data[98], data[99],
            data[100], data[101], data[102], data[103],
        ]);

        let drv2 = Self::parse_price_info(&data[104..136])?;

        let mut prod = [0u8; 32];
        prod.copy_from_slice(&data[136..168]);

        let mut next = [0u8; 32];
        next.copy_from_slice(&data[168..200]);

        let agg_pub_slot = u64::from_le_bytes([
            data[200], data[201], data[202], data[203],
            data[204], data[205], data[206], data[207],
        ]);

        let agg = Self::parse_price_info(&data[208..240])?;

        Ok(PriceAccount {
            magic,
            ver,
            atype,
            size,
            ptype,
            exponent,
            num,
            unused,
            curr_slot,
            valid_slot,
            twap,
            twac,
            drv1,
            drv2,
            prod,
            next,
            agg_pub_slot,
            agg,
        })
    }

    fn parse_price_info(data: &[u8]) -> Result<PriceInfo, String> {
        if data.len() < 32 {
            return Err("Insufficient data for PriceInfo".to_string());
        }

        let price = i64::from_le_bytes([
            data[0], data[1], data[2], data[3],
            data[4], data[5], data[6], data[7],
        ]);

        let conf = u64::from_le_bytes([
            data[8], data[9], data[10], data[11],
            data[12], data[13], data[14], data[15],
        ]);

        let status_raw = u32::from_le_bytes([data[16], data[17], data[18], data[19]]);
        let status = match status_raw {
            0 => PriceStatus::Unknown,
            1 => PriceStatus::Trading,
            2 => PriceStatus::Halted,
            3 => PriceStatus::Auction,
            _ => PriceStatus::Unknown,
        };

        let corp_act = u32::from_le_bytes([data[20], data[21], data[22], data[23]]);

        let pub_slot = u64::from_le_bytes([
            data[24], data[25], data[26], data[27],
            data[28], data[29], data[30], data[31],
        ]);

        Ok(PriceInfo {
            price,
            conf,
            status,
            corp_act,
            pub_slot,
        })
    }

    pub fn get_price_as_f64(&self) -> f64 {
        let base = self.agg.price as f64;
        let exponent = self.exponent as f64;
        base * 10f64.powf(exponent)
    }

    pub fn get_confidence_as_f64(&self) -> f64 {
        let base = self.agg.conf as f64;
        let exponent = self.exponent as f64;
        base * 10f64.powf(exponent)
    }

    pub fn is_valid(&self) -> bool {
        self.agg.price != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_info_parsing() {
        assert_eq!(std::mem::size_of::<PriceInfo>(), 32);
    }
}
