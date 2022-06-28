use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PeerType
{
    FullNode,
    LightNode,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Peer
{
    id: u64,
    ptype: PeerType,
    address: Ipv4Addr,
    connected_nodes: Vec<(u64, Ipv4Addr)>,
}

impl Peer
{
    pub fn new(id: u64, ptype: PeerType, address: Ipv4Addr) -> Peer
    {
        Peer {
            id,
            ptype,
            address,
            connected_nodes: vec![],
        }
    }

    pub fn with_nodes(
        id: u64,
        ptype: PeerType,
        address: Ipv4Addr,
        nodes: Vec<(u64, Ipv4Addr)>,
    ) -> Peer
    {
        Peer {
            id,
            ptype,
            address,
            connected_nodes: nodes,
        }
    }

    pub fn connect_node(&mut self, node: Peer)
    {
        self.connected_nodes.push((node.id, node.address));
    }

    pub fn id(&self) -> u64
    {
        self.id
    }

    pub fn set_address(&mut self, new_addr: Ipv4Addr)
    {
        self.address = new_addr;
    }

    pub fn ptype(&self) -> PeerType
    {
        self.ptype
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
