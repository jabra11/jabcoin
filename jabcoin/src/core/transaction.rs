use crate::core::address::Address;
use crate::core::crypto::Sha256Hash;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct Input
{
    addr: Address,
    value: u64,
}

impl Input
{
    fn new(addr: Address, value: u64) -> Input
    {
        Input { addr, value }
    }
}

impl Sha256Hash for Input
{
    fn hash(&self) -> Vec<u8>
    {
        let mut hasher = Sha256::new();

        hasher.update(self.value.to_be_bytes());
        hasher.update(self.addr.hash());
        hasher.finalize().to_vec()
    }
}

const MAX_OUT_ADDRESSES: usize = 100;

#[derive(Clone)]
pub struct Output
{
    addrs: Vec<(Address, u64)>,
}

impl Output
{
    fn new() -> Output
    {
        Output { addrs: vec![] }
    }

    fn with_addrs(addrs: Vec<(Address, u64)>) -> Result<Output, &'static str>
    {
        if addrs.len() > MAX_OUT_ADDRESSES
        {
            Err("too many addresses!")
        }
        else
        {
            Ok(Output { addrs })
        }
    }
}

impl Sha256Hash for Output
{
    fn hash(&self) -> Vec<u8>
    {
        let mut hasher = Sha256::new();

        for i in &self.addrs
        {
            hasher.update(&i.0.hash()[..]);
            hasher.update(i.1.to_be_bytes());
        }

        hasher.finalize().to_vec()
    }
}

#[derive(Clone)]
pub struct Transaction
{
    input: Input,
    output: Output,
}

impl Transaction
{
    pub fn new(input: Input, output: Output) -> Transaction
    {
        Transaction { input, output }
    }
}

impl Sha256Hash for Transaction
{
    fn hash(&self) -> Vec<u8>
    {
        let mut hasher = Sha256::new();

        hasher.update(&self.input.hash()[..]);
        hasher.update(&self.output.hash()[..]);

        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn hash_input()
    {
        let addr = Address::new();
        let input = Input::new(addr, 123);

        println!("{}", input.hash_str());
    }

    #[test]
    fn hash_output()
    {
        let mut v = vec![];
        for i in 0..10
        {
            let addr = Address::new();
            v.push((addr, i));
        }
        let output = Output::with_addrs(v).unwrap();

        println!("{}", output.hash_str());
    }

    #[test]
    fn hash_transaction()
    {
        let addr = Address::new();
        let input = Input::new(addr, 123);

        let mut v = vec![];
        for i in 0..10
        {
            let addr = Address::new();
            v.push((addr, i));
        }
        let output = Output::with_addrs(v).unwrap();

        let trx = Transaction::new(input, output);
        println!("{}", trx.hash_str());
    }
}
