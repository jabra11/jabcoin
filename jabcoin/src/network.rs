pub use protocol::{Header, Message};

pub mod protocol
{
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum Header
    {
        // broadcast a transaction
        BroadcastTransaction,

        // broadcast a block
        BroadcastBlock,

        // request a block (which one?)
        RequestBlock,

        // broadcast all peers
        BroadcastPeers,

        // request peers
        RequestPeers,

        // Register into the p2p-network
        Register,

        // Deregister from the p2p-network
        Deregister,

        // generic OK
        OK,

        // generic ERR
        ERR,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Message
    {
        pub header: Header,

        // serde_json format, not efficient but simple
        pub body: String,
    }

    impl Message
    {
        pub fn new() -> Message
        {
            Message::with_data(Header::OK, "")
        }

        pub fn with_data(header: Header, body: &str) -> Message
        {
            Message {
                header,
                body: String::from(body),
            }
        }
    }

    #[cfg(test)]
    mod tests
    {
        use super::*;
        use std::fs;

        use crate::core::{Address, Block};

        fn read_mock_address() -> Address
        {
            serde_json::from_str(&std::fs::read_to_string("etc/mock/address.json").unwrap())
                .unwrap()
        }

        #[test]
        fn serialize()
        {
            let miner = read_mock_address();
            let blk = Block::new(miner);
            let s = serde_json::to_string(&blk).unwrap();

            assert_eq!(blk, serde_json::from_str(&s).unwrap());
        }

        #[test]
        fn deserialize()
        {
            let s = fs::read_to_string("etc/mock/network/broadcast_block.json").unwrap();
            let _blk = serde_json::from_str::<Message>(&s).unwrap();
        }
    }
}
