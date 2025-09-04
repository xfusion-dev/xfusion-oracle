use sha2::{Digest, Sha256};
use serde_bytes::ByteBuf;
use crate::types::{Symbol, Price};

pub fn hash_data(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn build_merkle_tree(pairs: &[(Symbol, Price)]) -> Vec<u8> {
    if pairs.is_empty() {
        return vec![0; 32];
    }

    let mut leaves: Vec<[u8; 32]> = pairs
        .iter()
        .map(|(symbol, price)| {
            let mut data = Vec::new();
            data.extend_from_slice(symbol.as_bytes());
            data.extend_from_slice(&price.value.to_be_bytes());
            data.extend_from_slice(&price.timestamp.to_be_bytes());
            data.extend_from_slice(price.source.as_bytes());
            if let Some(conf) = price.confidence {
                data.extend_from_slice(&conf.to_be_bytes());
            }
            hash_data(&data)
        })
        .collect();

    while leaves.len() > 1 {
        let mut next_level = Vec::new();

        for chunk in leaves.chunks(2) {
            match chunk.len() {
                2 => {
                    let mut combined = Vec::new();
                    combined.extend_from_slice(&chunk[0]);
                    combined.extend_from_slice(&chunk[1]);
                    next_level.push(hash_data(&combined));
                }
                1 => {
                    next_level.push(chunk[0]);
                }
                _ => unreachable!(),
            }
        }

        leaves = next_level;
    }

    leaves[0].to_vec()
}

pub fn create_certified_snapshot(pairs: Vec<(Symbol, Price)>) -> (Vec<(Symbol, Price)>, ByteBuf) {
    let root_hash = build_merkle_tree(&pairs);
    (pairs, ByteBuf::from(root_hash))
}