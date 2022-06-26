use crate::core::block::Block;
use crate::core::crypto::Sha256Hash;
use std::collections::HashMap;

pub struct Blockchain
{
    length: u64,

    // holds the hash of the latest block
    head: Option<Vec<u8>>,
    blocks: HashMap<Vec<u8>, Block>,
}

impl Blockchain
{
    pub fn new() -> Blockchain
    {
        Blockchain {
            length: 0,
            head: None,
            blocks: HashMap::new(),
        }
    }

    pub fn head(&self) -> Option<&Block>
    {
        match &self.head
        {
            Some(h) => Some(&self.blocks[h]),
            None => None,
        }
    }

    pub fn get_blocks(&self) -> &HashMap<Vec<u8>, Block>
    {
        &self.blocks
    }

    pub fn len(&self) -> u64
    {
        self.length
    }

    fn verify(&self, block: &Block) -> Result<(), String>
    {
        // naive consensus
        if !block.hash_str().starts_with("000")
        {
            return Err(format!("hash is invalid: {}", block.hash_str()));
        }

        for i in block.transactions()
        {
            if let Some(sig) = &i.signature()
            {
                if let Err(e) = i.input().get_addr().verify_data(&i.hash_ignore_sig(), sig)
                {
                    return Err(format!(
                        "failed to verify signature for transaction: {} with error {}",
                        i.hash_str(),
                        e
                    ));
                }
            }
            else
            {
                return Err(format!(
                    "signature missing for transaction: {}",
                    i.hash_str()
                ));
            }

            let out_sum = i
                .output()
                .transactors()
                .iter()
                .fold(0, |acc, trxactor| acc + trxactor.get_value());

            if out_sum > i.input().get_value()
            {
                return Err(format!(
                    "insufficient input value in transaction: {}:\n {} < {}",
                    i.hash_str(),
                    i.input().get_value(),
                    out_sum
                ));
            }
        }

        Ok(())
    }

    pub fn append_block(&mut self, mut block: Block) -> Result<(), (String, Block)>
    {
        if let Err(e) = Blockchain::verify(self, &block)
        {
            Err((
                format!("block verification failed with error: {}", e),
                block,
            ))
        }
        else
        {
            if let Some(last) = self.head()
            {
                block.set_hash_prev(last.hash());
            }
            else
            {
                block.set_hash_prev(Vec::new());
            }

            let h = block.hash();
            self.head = Some(h.clone());
            self.blocks.insert(h, block);
            self.length += 1;

            Ok(())
        }
    }
}

impl TryFrom<Vec<Block>> for Blockchain
{
    type Error = String;

    fn try_from(blks: Vec<Block>) -> Result<Self, Self::Error>
    {
        let mut blkchain = Blockchain::new();
        let mut map: HashMap<&Vec<u8>, &Block> = HashMap::new();

        for blk in &blks
        {
            if blk.hash_prev().len() == 0
            {
                if blkchain.len() == 0
                {
                    if let Err((e, _)) = blkchain.append_block(blk.clone())
                    {
                        return Err(format!(
                            "failed to insert blk: {} with error {e}",
                            blk.hash_str()
                        ));
                    }
                }
                else
                {
                    return Err(String::from("multiple genesis blocks found"));
                }
            }
            else
            {
                map.insert(blk.hash_prev(), &blk);
            }
        }

        if blkchain.len() == 0
        {
            return Err(String::from("no genesis block found"));
        }

        for _ in 1..
        {
            let head = blkchain.head().unwrap();
            let mut next = map.get(&head.hash());
            if let Some(&blk) = next.take()
            {
                if let Err((e, _)) = blkchain.append_block(blk.clone())
                {
                    return Err(format!(
                        "failed to insert blk: {} with error {e}",
                        blk.hash_str()
                    ));
                }
            }
            else
            {
                break;
            }
        }
        Ok(blkchain)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::core::address::Address;
    use crate::core::crypto::generate_random_rsa_pair;
    use crate::core::transaction::{Input, Output, Transaction};

    fn read_mock_address() -> Address
    {
        serde_json::from_str(&std::fs::read_to_string("etc/mock/address.json").unwrap()).unwrap()
    }

    fn generate_random_transaction() -> Transaction
    {
        let rsa = generate_random_rsa_pair();
        let initiator = Address::with_key(rsa.to_public_key());

        let recipient = Address::generate_random();
        let inp = Input::new(initiator, 100);
        let outp = Output::with_addrs(vec![(recipient, 10)]).unwrap();

        let mut trx = Transaction::new(inp.clone(), outp.clone());

        let sig = rsa
            .sign(
                rsa::PaddingScheme::new_pkcs1v15_sign(None),
                &trx.hash_ignore_sig(),
            )
            .unwrap();

        trx.set_signature(sig);
        trx
    }

    #[test]
    fn from_vec()
    {
        let miner = read_mock_address();
        let mut gen = Block::new(miner.clone());
        let trx: Vec<Transaction> = (0..10).map(|_| generate_random_transaction()).collect();

        for i in &trx
        {
            gen.add_transaction(i.clone());
        }
        loop
        {
            if gen.hash_str().starts_with("000")
            {
                break;
            }
            gen.update_nounce()
        }

        let mut second = Block::new(miner.clone());
        for i in &trx
        {
            second.add_transaction(i.clone());
        }
        second.set_hash_prev(gen.hash());
        loop
        {
            if second.hash_str().starts_with("000")
            {
                break;
            }
            second.update_nounce()
        }

        let mut third = Block::new(miner.clone());
        for i in &trx
        {
            third.add_transaction(i.clone());
        }
        third.set_hash_prev(second.hash());
        loop
        {
            if third.hash_str().starts_with("000")
            {
                break;
            }
            third.update_nounce()
        }

        let _blkchain = Blockchain::try_from(vec![gen, second, third]).unwrap();
    }

    #[test]
    fn verify_block()
    {
        let trx = generate_random_transaction();

        let miner = read_mock_address();
        let mut blk = Block::new(miner);
        blk.add_transaction(trx);
        loop
        {
            if blk.hash_str().starts_with("000")
            {
                break;
            }
            blk.update_nounce()
        }

        let mut blkchain = Blockchain::new();
        blkchain.append_block(blk).unwrap();
    }
}
