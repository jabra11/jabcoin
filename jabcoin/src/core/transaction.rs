use crate::core::address::Address;
use crate::core::crypto::Sha256Hash;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transactor
{
    addr: Address,
    value: u64,
}

impl Transactor
{
    pub fn new(addr: Address, value: u64) -> Transactor
    {
        Transactor { addr, value }
    }

    pub fn get_addr(&self) -> &Address
    {
        &self.addr
    }

    pub fn get_value(&self) -> u64
    {
        self.value
    }
}

impl Sha256Hash for Transactor
{
    fn hash(&self) -> Vec<u8>
    {
        let mut hasher = Sha256::new();

        hasher.update(self.value.to_be_bytes());
        hasher.update(self.addr.hash());
        hasher.finalize().to_vec()
    }
}

pub type Input = Transactor;

const MAX_OUT_ADDRESSES: usize = 100;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Output
{
    addrs: Vec<Transactor>,
}

impl Output
{
    pub fn new() -> Output
    {
        Output { addrs: vec![] }
    }

    pub fn with_addrs(addrs: Vec<(Address, u64)>) -> Result<Output, &'static str>
    {
        if addrs.len() > MAX_OUT_ADDRESSES
        {
            Err("too many addresses!")
        }
        else
        {
            let mut v = vec![];
            for (addr, val) in addrs
            {
                v.push(Transactor::new(addr, val));
            }
            Ok(Output { addrs: v })
        }
    }

    pub fn transactors(&self) -> &Vec<Transactor>
    {
        &self.addrs
    }
}

impl Sha256Hash for Output
{
    fn hash(&self) -> Vec<u8>
    {
        let mut hasher = Sha256::new();

        for i in &self.addrs
        {
            hasher.update(&i.addr.hash()[..]);
            hasher.update(i.value.to_be_bytes());
        }
        hasher.finalize().to_vec()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction
{
    input: Input,
    output: Output,
    signature: Option<Vec<u8>>,
}

impl Transaction
{
    pub fn new(input: Input, output: Output) -> Transaction
    {
        Transaction {
            input,
            output,
            signature: None,
        }
    }

    pub fn with_signature(input: Input, output: Output, signature: Vec<u8>) -> Transaction
    {
        Transaction {
            input,
            output,
            signature: Some(signature),
        }
    }

    /// returns a hash of the transaction but without
    /// taking a possibly existent signature into account
    pub fn hash_ignore_sig(&self) -> Vec<u8>
    {
        // a bit inefficient because of the cloning,
        // should be fine for now though
        let tmp = Transaction::new(self.input.clone(), self.output.clone());
        tmp.hash()
    }

    pub fn input(&self) -> &Input
    {
        &self.input
    }

    pub fn output(&self) -> &Output
    {
        &self.output
    }

    pub fn set_signature(&mut self, sig: Vec<u8>)
    {
        self.signature.replace(sig);
    }

    pub fn signature(&self) -> Option<&Vec<u8>>
    {
        match &self.signature
        {
            Some(a) => Some(&a),
            None => None,
        }
    }
}

impl Sha256Hash for Transaction
{
    fn hash(&self) -> Vec<u8>
    {
        let mut hasher = Sha256::new();

        hasher.update(&self.input.hash()[..]);
        hasher.update(&self.output.hash()[..]);

        if let Some(sig) = &self.signature
        {
            hasher.update(&sig[..]);
        }

        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::core::crypto::generate_random_rsa_pair;

    #[test]
    fn hash_input()
    {
        let addr = Address::generate_random();
        let input = Input::new(addr, 123);

        println!("{}", input.hash_str());
    }

    #[test]
    fn hash_output()
    {
        let mut v = vec![];
        for i in 0..1
        {
            let addr = Address::generate_random();
            v.push((addr, i));
        }
        let output = Output::with_addrs(v).unwrap();

        println!("{}", output.hash_str());
    }

    #[test]
    fn hash_transaction()
    {
        let addr = Address::generate_random();
        let input = Input::new(addr, 123);

        let mut v = vec![];
        for i in 0..1
        {
            let addr = Address::generate_random();
            v.push((addr, i));
        }
        let output = Output::with_addrs(v).unwrap();

        let trx = Transaction::new(input, output);
        println!("{}", trx.hash_str());
    }

    #[test]
    fn verify_transaction()
    {
        // generate key pair
        let rsa = generate_random_rsa_pair();

        // generate input address
        let addr_input = Address::with_key(rsa.to_public_key());
        let inp = Input::new(addr_input, 10);

        // generate output addr
        let rsa2 = generate_random_rsa_pair();
        let addr_output = Address::with_key(rsa2.to_public_key());
        let out = Output::with_addrs(vec![(addr_output, 10)]).unwrap();

        let mut trx = Transaction::new(inp, out);

        let hash = trx.hash();

        trx.signature = Some(
            rsa.sign(rsa::padding::PaddingScheme::new_pkcs1v15_sign(None), &hash)
                .unwrap(),
        );
        trx.input
            .addr
            .verify_data(&hash, &trx.signature.as_ref().unwrap())
            .unwrap();

        // manipulate transaction
        trx.input.value = 5;
        let hash = trx.hash();

        // should now fail because signature shouldn't match
        // the hash
        trx.input
            .addr
            .verify_data(&hash, &trx.signature.as_ref().unwrap())
            .unwrap_err();
    }
}
