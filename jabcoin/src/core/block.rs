use crate::core::crypto::Sha256Hash;
use crate::core::transaction::Transaction;
use sha2::{Digest, Sha256};

#[derive(Clone)]
struct Nounce
{
    nounce: u64,
}

impl Nounce
{
    fn new() -> Nounce
    {
        Nounce { nounce: 0 }
    }

    fn incr(&mut self)
    {
        self.nounce += 1;
    }
}

#[derive(Clone)]
pub struct Block
{
    id: u64,
    hash_prev: Vec<u8>,
    nounce: Nounce,
    transactions: Vec<Transaction>,
}

impl Block
{
    /// construct an empty block
    pub fn new() -> Block
    {
        Block {
            id: 0,
            hash_prev: Vec::new(),
            nounce: Nounce::new(),
            transactions: Vec::new(),
        }
    }

    /// construct a block with information
    /// about its predecessor
    pub fn with_previous(prev: &Block) -> Block
    {
        Block {
            id: prev.id + 1,
            hash_prev: prev.hash(),
            nounce: Nounce::new(),
            transactions: Vec::new(),
        }
    }

    pub fn id(&self) -> u64
    {
        self.id
    }

    pub fn hash_prev(&mut self) -> &Vec<u8>
    {
        &self.hash_prev
    }
}

impl Sha256Hash for Block
{
    fn hash(&self) -> Vec<u8>
    {
        let mut hasher = Sha256::new();

        hasher.update(self.id.to_be_bytes());
        hasher.update(self.nounce.nounce.to_be_bytes());
        hasher.update(&self.hash_prev[..]);
        for transaction in &self.transactions
        {
            hasher.update(&transaction.hash()[..]);
        }
        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn generate_block()
    {
        let _block = Block::new();
        // maybe add more checks?
        todo!();
    }

    #[test]
    fn hash_block()
    {
        let block = Block::new();

        println!("{}", block.hash_str());
    }
}
