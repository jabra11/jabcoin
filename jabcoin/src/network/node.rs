use std::net::Ipv4Addr;

pub struct Node
{
    id: u64,
    address: Ipv4Addr,
    connected_nodes: Vec<Node>,
}

impl Node
{
    pub fn new() -> Node
    {
        Node {
            id: 0,
            address: Ipv4Addr::new(0, 0, 0, 0),
            connected_nodes: vec![],
        }
    }
}
