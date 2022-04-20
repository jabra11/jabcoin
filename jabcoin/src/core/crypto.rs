pub trait Sha256Hash
{
    fn hash(&self) -> Vec<u8>;

    fn hash_str(&self) -> String
    {
        self.hash().iter().map(|x| format!("{:x}", x)).collect()
    }
}
