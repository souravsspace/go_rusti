use crate::{errors::Result, transactions::Transaction};
use crypto::{digest::Digest, sha2::Sha256};
use log::info;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub const TARGET_HEXS: usize = 4;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    timestamp: u128,
    transactions: Vec<Transaction>,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
}

impl Block {
    pub fn get_transaction(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn new_genesis_block(tc: Transaction) -> Block {
        Block::new_block(vec![tc], String::new(), 0).unwrap()
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_prev_block(&self) -> String {
        self.prev_block_hash.clone()
    }

    pub fn new_block(
        data: Vec<Transaction>,
        prev_block_hash: String,
        height: usize,
    ) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();

        let mut block = Block {
            timestamp,
            transactions: data,
            prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
        block.run_proof_of_work()?;

        Ok(block)
    }

    fn run_proof_of_work(&mut self) -> Result<()> {
        info!("Mining block!");

        while !self.validate()? {
            self.nonce += 1;
        }

        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();

        Ok(())
    }

    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;

        let mut hasher = Sha256::new();
        hasher.input(&data[..]);

        let mut vec_zero: Vec<u8> = vec![];
        vec_zero.resize(TARGET_HEXS, b'0');
        Ok(hasher.result_str()[0..TARGET_HEXS] == String::from_utf8(vec_zero)?)
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            TARGET_HEXS,
            self.nonce,
        );

        let bytes: Vec<u8> = bincode::serialize(&content)?;
        Ok(bytes)
    }
}
