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

    // broadcast all nodes (to who? other node or p2p-network)
    BroadcastNodes,

    // request nodes (from who?)
    RequestNodes,

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

    // json format, not efficient but simple
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
    #[test]
    fn serialize()
    {
        todo!()
    }

    #[test]
    fn deserialize()
    {
        todo!();
    }
}
