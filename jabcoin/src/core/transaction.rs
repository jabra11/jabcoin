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
    pub fn new(addr: Address, value: u64) -> Input
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

    // a bit inefficient because of the cloning,
    // should be fine for now though
    pub fn hash_ignore_sig(&self) -> Vec<u8>
    {
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
        for i in 0..1
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
        for i in 0..1
        {
            let addr = Address::new();
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
        let mut rng = rand::thread_rng();
        let rsa = rsa::RsaPrivateKey::new(&mut rng, 1024).unwrap();

        // generate input address
        let addr_input = Address::with_key(rsa.to_public_key());
        let inp = Input::new(addr_input, 10);

        // generate output addr
        let rsa2 = rsa::RsaPrivateKey::new(&mut rng, 1024).unwrap();
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
