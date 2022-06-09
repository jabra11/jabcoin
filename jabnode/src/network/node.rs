use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Node
{
    id: u64,
    address: Ipv4Addr,
    connected_nodes: Vec<(u64, Ipv4Addr)>,
}

impl Node
{
    pub fn new(id: u64, address: Ipv4Addr) -> Node
    {
        Node {
            id,
            address,
            connected_nodes: vec![],
        }
    }

    pub fn with_nodes(id: u64, address: Ipv4Addr, nodes: Vec<(u64, Ipv4Addr)>) -> Node
    {
        Node {
            id,
            address,
            connected_nodes: nodes,
        }
    }

    pub fn connect_node(&mut self, node: Node)
    {
        self.connected_nodes.push((node.id, node.address));
    }

    pub fn id(&self) -> u64
    {
        self.id
    }

    pub fn address(&self) -> &Ipv4Addr
    {
        &self.address
    }

    pub fn connected_nodes(&self) -> &Vec<(u64, Ipv4Addr)>
    {
        &self.connected_nodes
    }
}
