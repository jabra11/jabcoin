use jabnode::node::{Config, Node};
use jabnode::KillToken;
use std::sync::Arc;

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

    let mut cfg = Config::with_default();
    cfg.blkpath = "etc/mock/blocks".into();

    let kt = Arc::new(KillToken::new());

    let node = Node::new(cfg, Arc::clone(&kt));

    ctrlc::set_handler(move || {
        kt.activate();
    })
    .expect("failed to set ctrl-c handler!");

    node.start();
}
