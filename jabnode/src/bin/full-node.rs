use jabcoin::core::crypto::Sha256Hash;
use jabcoin::core::{Block, Blockchain, Transaction};
use jabcoin::network::{Header, Message};

use jabnode::network::Connection;
use jabnode::threadpool::ThreadPool;

use serde_json::error::Category;

use log::{error, info, warn};

use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_new_transaction(trx: Transaction)
{
    info!("{:<30} {}", "received new transaction", trx.hash_str());
}

fn parse_msg(msg: Message)
{
    match msg.header
    {
        Header::BroadcastTransaction =>
        {
            match serde_json::from_str::<Transaction>(&msg.body)
            {
                Ok(t) =>
                {
                    handle_new_transaction(t);
                }
                Err(e) =>
                {
                    error!("{:<30} {e}", "failed to parse trx with error");
                }
            };
        }
        Header::BroadcastBlock =>
        {
            match serde_json::from_str::<Block>(&msg.body)
            {
                Ok(b) =>
                {
                    info!("{:<30} {}", "received new block", b.hash_str());
                }
                Err(e) =>
                {
                    error!("{:<30} {e}", "failed to parse block with error");
                }
            };
        }
        _ => todo!(),
    }
}

fn handle_connection(stream: TcpStream)
{
    info!("got a connection");

    let mut connection = Connection::new(stream.try_clone().unwrap(), stream);
    loop
    {
        // check msg validity
        match connection.read_msg()
        {
            Ok(m) => parse_msg(m),
            Err(e) => match e.classify()
            {
                Category::Eof =>
                {
                    warn!("reached EOF while reading msg");
                    break;
                }
                Category::Io =>
                {
                    error!("failed to read msg with i/o error: {e}");
                }
                Category::Syntax | Category::Data =>
                {
                    warn!("received invalid data with error: {e}");
                }
            },
        }
    }
}

fn restore_blockchain(dir_path: &str) -> Blockchain
{
    for i in fs::read_dir(dir_path).unwrap()
    {
        info!("{:?}", i.unwrap());
    }

    // todo!();

    let mut _chain = Blockchain::new();
    _chain
}

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

    info!("Starting up full-node.");
    info!("Restoring blockchain ..");
    let _bchain = restore_blockchain("blocks/");
    info!(".. done.");

    let listener = TcpListener::bind("127.0.0.1:27182").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming()
    {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    info!("Shutting down.");
}
