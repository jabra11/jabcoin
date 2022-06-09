use jabcoin::core::crypto::{generate_random_rsa_pair, PaddingScheme, RsaPrivateKey};
use jabcoin::core::Transaction;

pub struct Wallet
{
    key: RsaPrivateKey,
}

impl Wallet
{
    pub fn generate_random() -> Wallet
    {
        Wallet {
            key: generate_random_rsa_pair(),
        }
    }

    pub fn with_key(key: RsaPrivateKey) -> Wallet
    {
        Wallet { key }
    }

    pub fn sign(&self, trx: &mut Transaction)
    {
        let p = PaddingScheme::new_pkcs1v15_sign(None);
        trx.set_signature(self.key.sign(p, &trx.hash_ignore_sig()).unwrap());
    }
}
