use jabcoin::core::{crypto::Sha256Hash, Block, Transaction};
use jabcoin::network::{Header, Message};

use jabnode::network::Connection;

use log::info;

fn init_logger()
{
    #[cfg(debug_assertions)]
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "trace")
        .write_style_or("RUST_LOG_STYLE", "always");

    #[cfg(not(debug_assertions))]
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "info")
        .write_style_or("RUST_LOG_STYLE", "always");

    env_logger::init_from_env(env);
}

fn main()
{
    init_logger();

    info!("starting light-node");
    info!("setting up mock trx");

    let s_trx = std::fs::read_to_string("../jabcoin/etc/mock/transaction.json").unwrap();
    let trx = serde_json::from_str::<Transaction>(&s_trx).unwrap();
    let msg = Message::with_data(Header::BroadcastTransaction, &s_trx);

    let s_blk = std::fs::read_to_string("../jabcoin/etc/mock/block_large.json").unwrap();
    let blk = serde_json::from_str::<Block>(&s_blk).unwrap();
    let msg2 = Message::with_data(Header::BroadcastBlock, &s_blk);

    info!("sending trx: {}", trx.hash_str());
    info!("sending blk: {}", blk.hash_str());

    let tcpstream = std::net::TcpStream::connect("127.0.0.1:27182").unwrap();
    let mut connection = Connection::new(tcpstream.try_clone().unwrap(), tcpstream);

    loop
    {
        connection.write_msg(msg.clone()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        connection.write_msg(msg2.clone()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
