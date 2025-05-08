use crate::{blockchain::Blockchain, errors::Result};
use crypto::{digest::Digest, sha2::Sha256};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub vin: Vec<TxInput>,
    pub vout: Vec<TxOutput>,
}

impl Transaction {
    pub fn new_utxo(from: &str, to: &str, amount: i32, bc: &Blockchain) -> Result<Transaction> {
        let mut vin = Vec::new();
        let (total, spendable_outputs) = bc.find_spendable_outputs(from, amount);

        if total < amount {
            error!("Not enough balance!");
            return Err(failure::err_msg(format!(
                "Not enough balance: current balance: {}",
                total
            )));
        }

        for (txid, outs) in spendable_outputs {
            for &out_idx in &outs {
                vin.push(TxInput {
                    txid: txid.clone(),
                    vout: out_idx,
                    script_signature: from.to_string(),
                });
            }
        }

        let mut vout = vec![TxOutput {
            value: amount,
            script_pub_key: to.to_string(),
        }];

        if total > amount {
            vout.push(TxOutput {
                value: total - amount,
                script_pub_key: from.to_string(),
            });
        }

        let mut tx = Transaction {
            id: String::new(),
            vin,
            vout,
        };
        tx.set_id()?;

        Ok(tx)
    }

    pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction> {
        if data == String::from("") {
            data += &format!("Reward to '{}'", to);
        }

        let mut tx = Transaction {
            id: String::new(),
            vin: vec![TxInput {
                txid: String::new(),
                vout: -1,
                script_signature: data,
            }],
            vout: vec![TxOutput {
                value: 100,
                script_pub_key: to,
            }],
        };

        tx.set_id()?;
        Ok(tx)
    }

    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].txid.is_empty() && self.vin[0].vout == -1
    }

    fn set_id(&mut self) -> Result<()> {
        let mut hasher = Sha256::new();
        let data = bincode::serialize(self)?;

        hasher.input(&data);
        self.id = hasher.result_str();

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxInput {
    pub txid: String,
    pub vout: i32,
    pub script_signature: String,
}

impl TxInput {
    pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
        self.script_signature == unlocking_data
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxOutput {
    pub value: i32,
    pub script_pub_key: String,
}

impl TxOutput {
    pub fn can_be_unlock_with(&self, unlocking_data: &str) -> bool {
        self.script_pub_key == unlocking_data
    }
}
