use crate::core::crypto::Sha256Hash;
use crate::core::{Address, Transaction};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Block
{
    id: u64,
    nounce: Nounce,
    miner: Address,
    transactions: Vec<Transaction>,
    hash_prev: Vec<u8>,
}

impl Block
{
    /// construct an empty block
    pub fn new(miner: Address) -> Block
    {
        Block {
            id: 0,
            hash_prev: Vec::new(),
            miner,
            nounce: Nounce::new(),
            transactions: Vec::new(),
        }
    }

    /// construct a block with information
    /// about its predecessor
    pub fn with_previous(miner: Address, prev: &Block) -> Block
    {
        Block {
            id: prev.id + 1,
            hash_prev: prev.hash(),
            miner,
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

    pub fn get_miner(&self) -> &Address
    {
        &self.miner
    }

    pub fn transactions(&self) -> &Vec<Transaction>
    {
        &self.transactions
    }

    pub fn update_nounce(&mut self)
    {
        self.nounce.incr()
    }

    pub fn hash_prev(&self) -> &Vec<u8>
    {
        &self.hash_prev
    }

    pub fn set_hash_prev(&mut self, hash: Vec<u8>)
    {
        self.hash_prev = hash;
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
        hasher.update(&self.miner.hash());
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

    fn read_mock_address() -> Address
    {
        serde_json::from_str(&std::fs::read_to_string("etc/mock/address.json").unwrap()).unwrap()
    }

    #[test]
    fn generate_block()
    {
        let miner = read_mock_address();
        let _block = Block::new(miner);
    }

    #[test]
    fn hash_block()
    {
        let miner = read_mock_address();
        let block = Block::new(miner);
        println!("{}", block.hash_str());
    }

    fn generate_mock_block() -> Block
    {
        let miner = read_mock_address();
        let mut blk = Block::new(miner);

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
        let _serialized = serde_json::to_string(&blk).unwrap();
        //println!("{}", serialized);
    }

    #[test]
    fn deserialize_mock()
    {
        let data = std::fs::read_to_string("etc/mock/block.json").unwrap();
        serde_json::from_str::<Block>(&data).unwrap();
    }

    #[test]
    fn find_nounce()
    {
        let miner = read_mock_address();
        let mut block = Block::new(miner);

        let pred = |x: &String| return x.starts_with("0");

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
