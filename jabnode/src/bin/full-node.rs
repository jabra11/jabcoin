use jabcoin::core::crypto::Sha256Hash;
use jabcoin::core::{Block, Blockchain, Transaction};
use jabcoin::network::Header;

use jabnode::network::Connection;
use jabnode::threadpool::ThreadPool;

use log::{error, info};

use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(stream: TcpStream)
{
    info!("got a connection");

    let mut connection = Connection::new(stream.try_clone().unwrap(), stream);
    loop
    {
        match connection.read_msg()
        {
            Ok(m) => match m.header
            {
                Header::BroadcastTransaction =>
                {
                    match serde_json::from_str::<Transaction>(&m.body)
                    {
                        Ok(t) =>
                        {
                            info!("received new transaction: {}", t.hash_str());
                        }
                        Err(e) =>
                        {
                            error!("failed to parse trx with error: {e}");
                        }
                    };
                }
                Header::BroadcastBlock =>
                {
                    match serde_json::from_str::<Block>(&m.body)
                    {
                        Ok(b) =>
                        {
                            info!("received new block:       {}", b.hash_str());
                        }
                        Err(e) =>
                        {
                            error!("failed to parse trx with error: {e}");
                        }
                    };
                }
                _ => todo!(),
            },
            Err(e) =>
            {
                error!("failed to read msg with error: {e}");
                break;
            }
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
