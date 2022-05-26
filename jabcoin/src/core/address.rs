use crate::core::crypto::{
    generate_random_rsa_pair, BigUint, Digest, PublicKey, PublicKeyParts, RsaPublicKey, Sha256,
    Sha256Hash,
};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct Address
{
    key: RsaPublicKey,
}

impl Address
{
    pub fn new() -> Address
    {
        let empty = BigUint::new(vec![0]);
        Address {
            key: RsaPublicKey::new(empty.clone(), empty).unwrap(),
        }
    }

    pub fn with_key(key: RsaPublicKey) -> Address
    {
        Address { key }
    }

    /// randomly generate a address
    pub fn generate_random() -> Address
    {
        let rsa = generate_random_rsa_pair();
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
        let rsa = generate_random_rsa_pair();
        Address::with_key(rsa.to_public_key());
    }

    #[test]
    fn generate_hash()
    {
        let addr = Address::generate_random();

        println!("{:?}", addr.hash());
        println!("{}", addr.hash_str());
    }

    #[test]
    fn verify_data()
    {
        // generate key pair
        let rsa = generate_random_rsa_pair();

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

    fn to_string(a: &Address) -> String
    {
        serde_json::to_string(a).unwrap()
    }

    #[test]
    fn serialize()
    {
        let rsa = generate_random_rsa_pair();
        let a = Address::with_key(rsa.to_public_key());

        let s = to_string(&a);
        println!("{}", s);
    }

    #[test]
    fn deserialize()
    {
        let a = Address::generate_random();
        let s = to_string(&a);
        assert_eq!(a, serde_json::from_str(&s).unwrap());

        // should be distinct from a MOST LIKELY
        let a = Address::generate_random();
        assert_ne!(a, serde_json::from_str(&s).unwrap());
    }
}
