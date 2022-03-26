use crate::protocol::block::Block;

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

    fn verify(&self, _block: &Block) -> bool
    {
        true
    }

    pub fn append_block(&mut self, mut block: Block) -> Result<(), (&'static str, Block)>
    {
        if Blockchain::verify(self, &block)
        {
            if let Some(i) = &self.head
            {
                *(block.id_prev()) = i.id();
                self.head = Some(block);
            }
            else
            {
                *(block.id_prev()) = 0;
                self.head = Some(block);
            }
            Ok(())
        }
        else
        {
            Err(("block verification failed!", block))
        }
    }
}
