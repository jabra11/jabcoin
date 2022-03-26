use crate::protocol::address::Address;

#[derive(Clone)]
pub struct Transaction
{
    input: (Address, u64),
    output: Vec<(Address, u64)>,
}

impl Transaction
{
    pub fn new(input: (Address, u64), output: Vec<(Address, u64)>) -> Transaction
    {
        Transaction { input, output }
    }
}
