use crate::{Blockchain, Result};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::info;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

const TARGET_HEXT: usize = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
  timestamp: u128,
  transactions: String,
  prev_block_hash: String,
  hash: String,
  height: usize,
  nonce: i32,
}

impl Block {
  pub fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
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
    info!("Mining the block");
    let mut iter = 0;
    while !self.validate()? {
      iter += 1;
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
    let mut vec1 = vec![];
    vec1.resize(TARGET_HEXT, '0' as u8);

    Ok(&hasher.result_str()[0..TARGET_HEXT] == String::from_utf8(vec1)?)
  }

  fn prepare_hash_data(&self) -> Result<Vec<u8>> {
    let content = (
      self.prev_block_hash.clone(),
      self.transactions.clone(),
      self.timestamp,
      TARGET_HEXT,
      self.nonce,
    );
    let bytes = bincode::serialize(&content)?;
    Ok(bytes)
  }

  pub fn get_hash(&self) -> String {
    self.hash.clone()
  }

  pub fn get_prev_hash(&self) -> String {
    self.prev_block_hash.clone()
  }

  pub fn get_height(&self) -> usize {
    self.height.clone()
  }

  pub fn new_genesis_block() -> Block {
    Block::new_block(
      String::from("genesis block"),
      String::from("0000000000000000000000000000000000000000000000000000000000000001"),
      0,
    )
    .unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::remove_dir_all;

  #[test]
  fn test_blockchain() {
    // remove_dir_all(crate::BLOCKCHAIN_DATA_PATH);
    let mut b = Blockchain::new().unwrap();
    b.add_block("data".to_string());
    b.add_block("data2".to_string());
    b.add_block("data23".to_string());
    // remove_dir_all(crate::BLOCKCHAIN_DATA_PATH);

    dbg!(b.get_data());

    // for block in b.iter() {
    //   dbg!(block);
    // }
  }
}