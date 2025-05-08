use std::collections::HashMap;

use log::info;

use crate::{
    block::{Block, TARGET_HEXS},
    errors::Result,
    transactions::{Transaction, TxOutput},
};

const GENESIS_COINBASE_DATA: &str =
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

#[derive(Debug, Clone)]
pub struct Blockchain {
    current_hash: String,
    db: sled::Db,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain> {
        info!("Open blockchain!");

        let db = sled::open("data/blocks")?;
        let hash = db
            .get("LAST")?
            .expect("Must create a new block database first!");

        info!("Found block database!");

        let last_hash = String::from_utf8(hash.to_vec())?;

        Ok(Blockchain {
            current_hash: last_hash.clone(),
            db,
        })
    }

    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        info!("Creating a blockchain!");

        let db = sled::open("data/blocks")?;
        info!("Creating new block database!");

        let cbtx = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE_DATA))?;
        let genesis: Block = Block::new_genesis_block(cbtx);

        db.insert(genesis.get_hash(), bincode::serialize(&genesis)?)?;
        db.insert("LAST", genesis.get_hash().as_bytes())?;

        let bc = Blockchain {
            current_hash: genesis.get_hash(),
            db,
        };
        bc.db.flush()?;

        Ok(bc)
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<()> {
        let last_hash = self.db.get("LAST")?.unwrap();
        let new_block = Block::new_block(
            transactions,
            String::from_utf8(last_hash.to_vec())?,
            TARGET_HEXS,
        )?;

        self.db
            .insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.db.flush()?;
        self.current_hash = new_block.get_hash();

        Ok(())
    }

    pub fn find_unspent_tansactions(&self, address: &str) -> Vec<Transaction> {
        let mut spent_txos: HashMap<String, Vec<i32>> = HashMap::new();
        let mut unspend_txs: Vec<Transaction> = Vec::new();

        for block in self.iter() {
            for tx in block.get_transaction() {
                for index in 0..tx.vout.len() {
                    if let Some(ids) = spent_txos.get(&tx.id) {
                        if ids.contains(&(index as i32)) {
                            continue;
                        }
                    }

                    if tx.vout[index].can_be_unlock_with(address) {
                        unspend_txs.push(tx.to_owned());
                    }

                    if !tx.is_coinbase() {
                        for i in &tx.vin {
                            if i.can_unlock_output_with(address) {
                                match spent_txos.get_mut(&i.txid) {
                                    Some(v) => v.push(i.vout),
                                    None => {
                                        spent_txos.insert(i.txid.clone(), vec![i.vout]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        unspend_txs
    }

    pub fn find_utxo(&self, address: &str) -> Vec<TxOutput> {
        let mut utxos = Vec::<TxOutput>::new();
        let unpend_txs = self.find_unspent_tansactions(address);

        for tx in unpend_txs {
            for out in &tx.vout {
                if out.can_be_unlock_with(&address) {
                    utxos.push(out.clone());
                }
            }
        }

        utxos
    }

    pub fn find_spendable_outputs(
        &self,
        address: &str,
        amount: i32,
    ) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut accumulated: i32 = 0;
        let unspend_txs = self.find_unspent_tansactions(address);

        for tx in unspend_txs {
            for index in 0..tx.vout.len() {
                if tx.vout[index].can_be_unlock_with(address) && accumulated < amount {
                    match unspent_outputs.get_mut(&tx.id) {
                        Some(v) => v.push(index as i32),
                        None => {
                            unspent_outputs.insert(tx.id.clone(), vec![index as i32]);
                        }
                    }
                    accumulated += tx.vout[index].value;

                    if accumulated >= amount {
                        return (accumulated, unspent_outputs);
                    }
                }
            }
        }

        (accumulated, unspent_outputs)
    }

    pub fn iter(&self) -> BlockchainIter {
        BlockchainIter {
            current_hash: self.current_hash.clone(),
            bc: self,
        }
    }
}

pub struct BlockchainIter<'a> {
    current_hash: String,
    bc: &'a Blockchain,
}

impl Iterator for BlockchainIter<'_> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encode_block) = self.bc.db.get(&self.current_hash) {
            return match encode_block {
                Some(b) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_block();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }

        None
    }
}
