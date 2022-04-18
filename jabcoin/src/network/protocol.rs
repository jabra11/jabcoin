enum Header
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

struct Message
{
    header: Header,

    // key:value pairs, not efficient but simple
    data: Vec<(String, String)>,
}
