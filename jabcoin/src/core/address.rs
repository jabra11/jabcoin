use crate::core::crypto::Sha256Hash;
use rsa::{PublicKey, PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Deserialize, Serialize)]
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
        let rsa = RsaPrivateKey::new(&mut rng, 1024).unwrap();
        Address::with_key(rsa.to_public_key())
    }

    pub fn get_key(&self) -> &RsaPublicKey
    {
        return &self.key;
    }

    pub fn verify_data(&self, data: &[u8], sig: &[u8]) -> Result<(), String>
    {
        let padding_scheme = rsa::PaddingScheme::new_pkcs1v15_sign(None);
        match self.key.verify(padding_scheme, data, sig)
        {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
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
        let rsa = rsa::RsaPrivateKey::new(&mut rng, 1024).unwrap();
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
        // generate key pair
        let mut rng = rand::thread_rng();
        let rsa = rsa::RsaPrivateKey::new(&mut rng, 1024).unwrap();

        // generate address from public keypair
        let addr = Address::with_key(rsa.to_public_key());

        let mut data: Vec<u8> = vec![1; 32];

        let sig = rsa
            .sign(
                rsa::padding::PaddingScheme::new_pkcs1v15_sign(None),
                &data[..],
            )
            .unwrap();
        println!("{:?}", sig);
        addr.verify_data(&data[..], &sig[..]).unwrap();

        // change input
        data.push(1);

        // verification should fail now
        let failed_result = addr.verify_data(&data[..], &sig[..]);

        match failed_result
        {
            Ok(_) => panic!("verification succeeded with corrupted data"),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn serialize()
    {
        let mut rng = rand::thread_rng();
        let rsa = rsa::RsaPrivateKey::new(&mut rng, 1024).unwrap();
        let a = Address::with_key(rsa.to_public_key());

        let s = serde_json::to_string_pretty(&a).unwrap();
        println!("{}", s);
    }

    #[test]
    fn deserialize()
    {
        //let mut rng = rand::thread_rng();
        //let rsa = rsa::RsaPrivateKey::new(&mut rng, 1024).unwrap();
        //Address::with_key(rsa.to_public_key());
    }
}
