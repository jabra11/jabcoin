// keep this for now to silence stupid warnings
#![allow(dead_code)]

use jabcoin::core::crypto::{generate_random_rsa_pair, Sha256Hash};
use jabcoin::core::{Address, Block, Input, Output, Transaction};

use rand::{thread_rng, Rng};

pub fn read_mock_address() -> Address
{
    serde_json::from_str(&std::fs::read_to_string("etc/mock/address.json").unwrap()).unwrap()
}

pub fn setup_mock_block(
    count_transactions: u64,
    count_distinct_transactors: u64,
    prefix: &str,
) -> Block
{
    let miner = read_mock_address();
    let mut blk = Block::new(miner);

    let mut transactors = Vec::with_capacity(count_distinct_transactors as usize);

    for _ in 0..count_distinct_transactors
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
        if blk.hash_str().starts_with(prefix)
        {
            break;
        }
        blk.update_nounce()
    }
    blk
}
