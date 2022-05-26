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
                *block.hash_prev() = last.hash();
            }
            else
            {
                *block.hash_prev() = Vec::new();
            }

            let h = block.hash();
            self.head = Some(h.clone());
            self.blocks.insert(h, block);
            self.length += 1;

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::core::address::Address;
    use crate::core::crypto::generate_random_rsa_pair;
    use crate::core::transaction::{Input, Output, Transaction};

    #[test]
    fn verify_block()
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

        let mut blk = Block::new();
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
