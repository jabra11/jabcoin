use crate::core::crypto::Sha256Hash;
use crate::core::transaction::Transaction;
use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct Address
{
    key: RsaPublicKey,
}

impl Address
{
    pub fn with_key(key: RsaPublicKey) -> Address
    {
        Address { key }
    }

    /// randomly generate a address
    pub fn new() -> Address
    {
        // do we really want to randomly generate an address
        // with the new constructor?
        let mut rng = rand::thread_rng();
        let rsa = RsaPrivateKey::new(&mut rng, 256).unwrap();
        Address::with_key(rsa.to_public_key())
    }

    pub fn get_key(&self) -> &RsaPublicKey
    {
        return &self.key;
    }

    pub fn verify_transaction(&mut self, tx: &mut Transaction)
    {
        todo!();
    }
}

impl Sha256Hash for Address
{
    fn hash(&self) -> Vec<u8>
    {
        let mut hasher = Sha256::new();

        let n_byts = self.key.n().to_bytes_be();
        let e_byts = self.key.e().to_bytes_be();

        hasher.update(&n_byts[..]);
        hasher.update(&e_byts[..]);

        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn generate_address()
    {
        let mut rng = rand::thread_rng();
        let rsa = rsa::RsaPrivateKey::new(&mut rng, 256).unwrap();
        Address::with_key(rsa.to_public_key());
    }

    #[test]
    fn generate_hash()
    {
        let addr = Address::new();

        println!("{:?}", addr.hash());
        println!("{}", addr.hash_str());
    }

    #[test]
    fn verify_data()
    {
        todo!();
    }
}
