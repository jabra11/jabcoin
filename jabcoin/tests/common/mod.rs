use jabcoin::core::crypto::{generate_random_rsa_pair, Sha256Hash};
use jabcoin::core::{Address, Block, Input, Output, Transaction};
use jabcoin::network::Node;

use rand::{thread_rng, Rng};

pub fn setup_mock_block(count_transactions: u64, count_distinct_transactors: u64) -> Block
{
    let mut blk = Block::new();

    let mut transactors = Vec::with_capacity(count_distinct_transactors as usize);

    for i in 0..count_distinct_transactors
    {
        let rsa_pair = generate_random_rsa_pair();
        let pubaddr = Address::with_key(rsa_pair.to_public_key());
        transactors.push((rsa_pair, pubaddr));
    }

    for _ in 0..count_transactions
    {
        let mut r = thread_rng();
        let initiator: usize = r.gen_range(0..count_distinct_transactors) as usize;

        // let's allow addresses to transact with themselves
        let recipient: usize = r.gen_range(0..count_distinct_transactors) as usize;

        let input = Input::new(transactors[initiator].1.clone(), r.gen_range(1..=10000));
        let output =
            Output::with_addrs(vec![(transactors[recipient].1.clone(), input.get_value())])
                .unwrap();

        let mut trx = Transaction::new(input, output);

        let padding = rsa::PaddingScheme::new_pkcs1v15_sign(None);

        trx.set_signature(
            transactors[initiator]
                .0
                .sign(padding, &trx.hash_ignore_sig())
                .unwrap(),
        );
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
