use crate::protocol::transaction::Transaction;

#[derive(Clone)]
pub struct Block
{
    id: u64,
    id_prev: u64,
    transactions: Vec<Option<Transaction>>,
}

impl Block
{
    pub fn new(id: u64, id_prev: u64) -> Block
    {
        Block {
            id,
            id_prev,
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
