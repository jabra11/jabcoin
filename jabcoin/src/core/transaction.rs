use crate::core::address::Address;

#[derive(Clone, Hash)]
struct Input
{
    addr: Address,
    value: u64,
}

impl Input
{
    fn new(addr: Address, value: u64) -> Input
    {
        Input { addr, value }
    }
}

const MAX_OUT_ADDRESSES: usize = 100;

#[derive(Clone, Hash)]
struct Output
{
    addrs: Vec<(Address, u64)>,
}

impl Output
{
    fn new() -> Output
    {
        Output { addrs: vec![] }
    }

    fn with_addrs(addrs: Vec<(Address, u64)>) -> Result<Output, &'static str>
    {
        if addrs.len() > MAX_OUT_ADDRESSES
        {
            Err("too many addresses!")
        }
        else
        {
            Ok(Output { addrs })
        }
    }
}

#[derive(Clone, Hash)]
pub struct Transaction
{
    input: Input,
    output: Output,
}

impl Transaction
{
    pub fn new(input: (Address, u64), output: Vec<(Address, u64)>) -> Transaction
    {
        let inp = Input::new(input.0, input.1);
        let out = Output::with_addrs(output);

        let out = match out
        {
            Ok(o) => o,
            Err(e) =>
            {
                panic!("err: {}", e);
            }
        };

        Transaction {
            input: inp,
            output: out,
        }
    }
}
