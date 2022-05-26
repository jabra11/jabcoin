use jabcoin::core::crypto::Sha256Hash;
use jabcoin::core::{Address, Block, Input, Output, Transaction};
use jabcoin::network::Node;

pub fn setup_mock_block() -> Block
{
    let mut blk = Block::new();

    let initiator = Address::generate_random();
    let recipient = Address::generate_random();

    for i in 1..100
    {
        let inp = Input::new(initiator.clone(), i);
        let outp = Output::with_addrs(vec![(recipient.clone(), i)]).unwrap();
        let trx = Transaction::with_signature(inp, outp, vec![123, 12, 31, 23, 123]);
        blk.add_transaction(trx);
    }

    loop
    {
        if blk.hash_str().starts_with("0")
        {
            break;
        }
        blk.update_nounce()
    }
    blk
}

pub fn setup_mock_node(
    id: u64,
    ipaddr: Option<(u8, u8, u8, u8)>,
    con_nodes: Option<Vec<Node>>,
) -> Node
{
    let mut n = Node::new(id, std::net::Ipv4Addr::new(127, 0, 0, 1));
    if let Some((a, b, c, d)) = ipaddr
    {
        n = Node::new(id, std::net::Ipv4Addr::new(a, b, c, d));
    }

    if let Some(nodes) = con_nodes
    {
        for i in nodes
        {
            n.connect_node(i.clone());
        }
    }
    n
}
