pub enum Header
{
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
}

pub struct Message
{
    pub header: Header,

    // key:value pairs, not efficient but simple
    pub data: Vec<(String, String)>,
}
