use crate::core::block::Block;

pub struct Blockchain
{
    length: u64,
    head: Option<Block>,
}

impl Blockchain
{
    pub fn new() -> Blockchain
    {
        Blockchain {
            length: 0,
            head: None,
        }
    }

    pub fn head(&self) -> Option<&Block>
    {
        match &self.head
        {
            Some(h) => Some(h),
            None => None,
        }
    }

    pub fn len(&self) -> u64
    {
        self.length
    }

    pub fn verify(&self, _block: &Block) -> bool
    {
        // what constitutes a valid block?
        todo!();
    }

    pub fn append_block(&mut self, block: Block) -> Result<(), (&'static str, Block)>
    {
        if Blockchain::verify(self, &block)
        {
            todo!();
        }
        else
        {
            Err(("block verification failed!", block))
        }
    }
}
