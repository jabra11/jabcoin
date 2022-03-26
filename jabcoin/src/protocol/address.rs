#[derive(Copy, Clone)]
pub struct Address
{
    id: u64,
    // should probably include a public key to
    // support ownership
}

impl Address
{
    fn new(val: u64) -> Address
    {
        Address { id: val }
    }
}
