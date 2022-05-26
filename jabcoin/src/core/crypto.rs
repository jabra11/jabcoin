// reexport crate-foreign types for convenience
pub use rsa::{BigUint, PublicKey, PublicKeyParts, RsaPrivateKey, RsaPublicKey};
pub use sha2::{Digest, Sha256};

pub trait Sha256Hash
{
    fn hash(&self) -> Vec<u8>;

    fn hash_str(&self) -> String
    {
        self.hash().iter().map(|x| format!("{:02x}", x)).collect()
    }
}

pub fn generate_random_rsa_pair() -> RsaPrivateKey
{
    let mut rng = rand::thread_rng();
    RsaPrivateKey::new(&mut rng, 1024).unwrap()
}
