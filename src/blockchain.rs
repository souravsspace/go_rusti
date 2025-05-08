use crate::{
    block::{Block, TARGET_HEXS},
    errors::Result,
};

#[derive(Debug, Clone)]
pub struct Blockchain {
    current_hash: String,
    db: sled::Db,
}

impl Blockchain {
    pub fn create_blockchain() -> Result<Blockchain> {
        let db = sled::open("data/blocks")?;

        match db.get("LAST")? {
            Some(hash) => {
                let last_hash = String::from_utf8(hash.to_vec())?;

                Ok(Blockchain {
                    current_hash: last_hash,
                    db,
                })
            }
            None => {
                let block = Block::new_genesis_block();

                db.insert(block.get_hash(), bincode::serialize(&block)?)?;
                db.insert("LAST", block.get_hash().as_bytes())?;

                let bc = Blockchain {
                    current_hash: block.get_hash(),
                    db,
                };
                bc.db.flush()?;

                Ok(bc)
            }
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()> {
        let last_hash = self.db.get("LAST")?.unwrap();
        let new_block =
            Block::new_block(data, String::from_utf8(last_hash.to_vec())?, TARGET_HEXS)?;

        self.db
            .insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;

        self.current_hash = new_block.get_hash();

        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain() {
        let mut bc = Blockchain::create_blockchain().unwrap();

        bc.add_block(String::from("Sourav")).unwrap();
        bc.add_block(String::from("Riya")).unwrap();

        for block in bc.iter() {
            dbg!("{}", block);
        }
    }
}
