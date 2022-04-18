use crate::core::transaction::Transaction;
use rsa::RsaPublicKey;

#[derive(Clone, Hash)]
pub struct Address
{
    pub id: RsaPublicKey,
}

impl Address
{
    pub fn with_key(key: &mut RsaPublicKey) -> Address
    {
        Address { id: key.clone() }
    }

    /// randomly generate a address
    pub fn new() -> Address
    {
        let mut rng = rand::thread_rng();
        let rsa = rsa::RsaPrivateKey::new(&mut rng, 256).unwrap();
        Address::with_key(&mut rsa.to_public_key())
    }

    fn verify_transaction(&mut self, tx: &mut Transaction)
    {
        let data: [u8; 4] = [0, 1, 2, 3];
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
        let addr = Address::with_key(&mut rsa.to_public_key());
    }

    #[test]
    fn verify_data()
    {
        let mut addr = Address::new();

        let data: [u8; 4] = [0, 1, 2, 3];
        //addr.id.sign(rsa::PaddingScheme::PKCS1v15Sign { hash: None } ,&data[..]);
    }
}
