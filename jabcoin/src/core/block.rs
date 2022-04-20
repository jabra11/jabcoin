use crate::core::transaction::Transaction;

#[derive(Clone)]
struct Nounce
{
    nounce: u64,
}

impl Nounce
{
    fn new() -> Nounce
    {
        Nounce { nounce: 0 }
    }

    fn incr(&mut self)
    {
        self.nounce += 1;
    }
}

#[derive(Clone)]
pub struct Block
{
    id: u64,
    id_prev: u64,
    nounce: Nounce,
    transactions: Vec<Option<Transaction>>,
}

impl Block
{
    pub fn new(id: u64, id_prev: u64) -> Block
    {
        Block {
            id,
            id_prev,
            nounce: Nounce::new(),
            transactions: Vec::new(),
        }
    }

    pub fn id(&self) -> u64
    {
        self.id
    }

    pub fn id_prev(&mut self) -> &mut u64
    {
        &mut self.id_prev
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn generate_block()
    {
        todo!();
    }

    #[test]
    fn hash_block()
    {
        todo!();
    }
}
