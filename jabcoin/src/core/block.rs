use crate::core::crypto::Sha256Hash;
use crate::core::transaction::Transaction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
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

    pub fn add_transaction(&mut self, trx: Transaction)
    {
        // check validity? todo
        self.transactions.push(trx);
    }

    pub fn transactions(&self) -> &Vec<Transaction>
    {
        &self.transactions
    }

    pub fn update_nounce(&mut self)
    {
        self.nounce.incr()
    }

    pub fn hash_prev(&mut self) -> &mut Vec<u8>
    {
        &mut self.hash_prev
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
    use crate::core::address::Address;
    use crate::core::transaction::{Input, Output, Transaction};

    #[test]
    fn generate_block()
    {
        let _block = Block::new();
    }

    #[test]
    fn hash_block()
    {
        let block = Block::new();
        println!("{}", block.hash_str());
    }

    fn generate_mock_block() -> Block
    {
        let mut blk = Block::new();

        let initiator = Address::generate_random();
        let recipient = Address::generate_random();

        for i in 1..100
        {
            let inp = Input::new(initiator.clone(), i);
            let outp = Output::with_addrs(vec![(recipient.clone(), i)]).unwrap();
            let trx = Transaction::with_signature(inp, outp, vec![123, 12, 31, 23, 123]);
            blk.add_transaction(trx);
        }

        loop
        {
            if blk.hash_str().starts_with("0")
            {
                break;
            }
            blk.update_nounce()
        }
        blk
    }

    #[test]
    fn serialize()
    {
        let blk = generate_mock_block();
        let serialized = serde_json::to_string(&blk).unwrap();
        println!("{}", serialized);
    }

    #[test]
    fn find_nounce()
    {
        let mut block = Block::new();

        let pred = |x: &String| return x.starts_with("ab");

        let mut hashes = vec![];
        for _ in 0..100
        {
            hashes.push(block.hash_str());
            block.nounce.incr();
        }

        let good: Vec<String> = hashes.clone().into_iter().filter(pred).collect();
        assert!(good.len() > 0);
    }
}
