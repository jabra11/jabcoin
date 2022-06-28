use crate::network::{Connection, Peer, PeerType};
use crate::threadpool::ThreadPool;

use jabcoin::core::{crypto::Sha256Hash, Address, Block, Blockchain, Transaction};
use jabcoin::network::{Header, Message};

use log::{error, info, warn};

use serde_json::error::Category;

use std::collections::{HashMap, VecDeque};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

const PORT: u16 = 27182;

pub struct Config
{
    /// path to the block disk store
    pub blkpath: String,

    /// cache transactions inside the blockchain
    pub build_cache: bool,

    /// compute hashes to advance the blockchain
    pub mine: bool,
    pub count_chain_workers: usize,

    /// listen and process network communication
    pub listen_communication: bool,
    pub count_comm_workers: usize,

    /// vector of known peers
    pub peers: Vec<Peer>,

    pub peer: Peer,
}

impl Config
{
    pub fn new(
        blkpath: String,
        build_cache: bool,
        mine: bool,
        count_chain_workers: usize,
        listen_communication: bool,
        count_comm_workers: usize,
        peers: Vec<Peer>,
        peer: Peer,
    ) -> Config
    {
        Config {
            blkpath,
            build_cache,
            mine,
            listen_communication,
            count_comm_workers,
            count_chain_workers,
            peers,
            peer,
        }
    }

    pub fn with_default() -> Config
    {
        let slf = Peer::new(0, PeerType::FullNode, [127, 0, 0, 1].into());
        let p = Peer::new(1, PeerType::FullNode, [192, 168, 208, 115].into());
        let peers = vec![p];

        Config {
            blkpath: String::from("etc/blocks/"),
            build_cache: true,
            mine: true,
            listen_communication: true,
            count_comm_workers: 4,
            count_chain_workers: 1,
            peers,
            peer: slf,
        }
    }
}

struct State
{
    /// a cache of the state generated by  
    /// the blockchain
    economy: HashMap<Address, u64>,

    /// queue of freestanding transactions
    trx_queue: VecDeque<Transaction>,

    chain: Blockchain,
    peer: Peer,
    peers: Vec<Peer>,
}

pub struct Node
{
    state: Mutex<State>,
    cfg: Mutex<Config>,
}

impl Node
{
    pub fn new(cfg: Config) -> Node
    {
        let peers = cfg.peers.clone();
        let peer = cfg.peer.clone();

        let state = State {
            economy: HashMap::new(),
            trx_queue: VecDeque::new(),
            chain: Blockchain::new(),
            peers,
            peer,
        };

        Node {
            state: Mutex::new(state),
            cfg: Mutex::new(cfg),
        }
    }

    fn mine(self: Arc<Self>)
    {
        info!("starting to mine");
        info!("stopping to mine");
    }

    fn register_to_peer(self: Arc<Self>, peer: Ipv4Addr)
    {
        info!("trying to register to {peer}");
        let slf_str = {
            let state = self.state.lock().unwrap();
            serde_json::to_string(&state.peer).unwrap()
        };

        let msg = Message::with_data(Header::Register, &slf_str);
        let socketaddr = SocketAddrV4::new(peer, PORT);

        if let Ok(conn) = TcpStream::connect(socketaddr)
        {
            let mut conn = Connection::new(conn);
            if let Ok(_) = conn.write_msg(msg)
            {
                info!("registered to {peer}");
            }
            else
            {
                warn!("failed to write message to {peer}");
            }
        }
        else
        {
            warn!("failed to connect to {peer}");
        }
    }

    fn handle_new_transaction(self: Arc<Self>, trx: Transaction)
    {
        info!("{:<30} {}", "received new transaction", trx.hash_str());

        if trx.check_validity()
        {
            let q = &mut self.state.lock().unwrap().trx_queue;

            if let Some(_) = q.iter().find(|t| **t == trx)
            {
                info!("{:<30} {}", "already in queue.", trx.hash_str());
            }
            else
            {
                q.push_back(trx.clone());

                info!(
                    "{:<30} {} {}",
                    "queued at position ",
                    trx.hash_str(),
                    q.len()
                );
            }
        }
        else
        {
            warn!("{:<30} {}", "invalid transaction!", trx.hash_str());
        }
    }

    fn handle_new_block(self: Arc<Self>, blk: Block, peer: &Ipv4Addr)
    {
        info!("{peer}: {:<30} {}", "received new block", blk.hash_str());
    }

    fn parse_msg(self: Arc<Self>, msg: Message, peer_addr: &Ipv4Addr)
    {
        match msg.header
        {
            Header::Register =>
            {
                info!("{peer_addr}: received a register request");
                let peer = match serde_json::from_str::<Peer>(&msg.body)
                {
                    Ok(p) => Some(p),
                    Err(e) =>
                    {
                        warn!(
                            "{peer_addr}: {:<30} {e}",
                            "failed to parse request with error"
                        );
                        None
                    }
                };

                if let Some(mut peer) = peer
                {
                    let mut state = self.state.lock().unwrap();
                    peer.set_address(peer_addr.clone());
                    if !state.peers.contains(&peer)
                    {
                        let peers_str = serde_json::to_string(&state.peers).unwrap();

                        state.peers.push(peer);

                        let msg = Message::with_data(Header::BroadcastNodes, &peers_str);
                        info!("sharing peers with {peer_addr}");

                        let conn =
                            TcpStream::connect(SocketAddrV4::new(peer_addr.clone(), PORT)).unwrap();
                        let mut conn = Connection::new(conn);
                        conn.write_msg(msg).unwrap();
                    }
                    else
                    {
                        info!("{peer_addr}: already registered");
                    }
                }
            }
            Header::BroadcastNodes =>
            {
                info!("{peer_addr}: received new peers.");
                let mut new_peers = serde_json::from_str::<Vec<Peer>>(&msg.body).unwrap();

                let slf_str = {
                    // only hold the mutex lock in this scope
                    let state = self.state.lock().unwrap();
                    new_peers = new_peers
                        .into_iter()
                        .filter(|p| !state.peers.contains(p))
                        .collect();

                    serde_json::to_string::<Peer>(&state.peer).unwrap()
                };

                let mut good_peers = vec![];

                for i in new_peers
                {
                    let msg = Message::with_data(Header::Register, &slf_str);
                    let socketaddr = SocketAddrV4::new(i.address().clone(), PORT);
                    if let Ok(conn) = TcpStream::connect(socketaddr)
                    {
                        let mut conn = Connection::new(conn);
                        if let Ok(_) = conn.write_msg(msg)
                        {
                            info!("registered to {}", i.address());
                            good_peers.push(i);
                        }
                    }
                }

                let mut state = self.state.lock().unwrap();

                for i in good_peers
                {
                    state.peers.push(i);
                }
            }
            Header::RequestNodes =>
            {
                info!("{peer_addr}: received request nodes request.");

                let state = self.state.lock().unwrap();
                let peers = serde_json::to_string::<Vec<Peer>>(&state.peers).unwrap();

                let msg = Message::with_data(Header::BroadcastNodes, &peers);
                let socketaddr = std::net::SocketAddrV4::new(peer_addr.clone(), PORT);
                let conn = TcpStream::connect(socketaddr).unwrap();

                let mut conn = Connection::new(conn);
                conn.write_msg(msg).unwrap();
            }
            Header::Deregister =>
            {
                info!("{peer_addr}: received deregister request.");

                let mut state = self.state.lock().unwrap();

                let idx = state.peers.iter().position(|p| p.address() == peer_addr);

                if let Some(idx) = idx
                {
                    state.peers.swap_remove(idx);
                    info!("removed node {peer_addr}");
                }
                else
                {
                    warn!("Did not find {peer_addr} in memory");
                }
            }
            Header::BroadcastTransaction =>
            {
                match serde_json::from_str::<Transaction>(&msg.body)
                {
                    Ok(t) =>
                    {
                        self.handle_new_transaction(t);
                    }
                    Err(e) =>
                    {
                        warn!("{peer_addr}: {:<30} {e}", "failed to parse trx with error");
                    }
                };
            }
            Header::BroadcastBlock =>
            {
                match serde_json::from_str::<Block>(&msg.body)
                {
                    Ok(b) =>
                    {
                        self.handle_new_block(b, peer_addr);
                    }
                    Err(e) =>
                    {
                        warn!(
                            "{peer_addr}: {:<30} {e}",
                            "failed to parse block with error"
                        );
                    }
                };
            }
            _ => todo!(),
        }
    }

    fn handle_connection(self: Arc<Self>, mut conn: Connection)
    {
        let peer = conn.get_peer_addr();
        info!("{peer}: new connection.");

        loop
        {
            // check msg validity
            match conn.read_msg()
            {
                Ok(m) => Node::parse_msg(Arc::clone(&self), m, &peer),
                Err(e) => match e.classify()
                {
                    Category::Eof =>
                    {
                        break;
                    }
                    Category::Io =>
                    {
                        error!("{peer}: failed to read msg with i/o error: {e}");
                    }
                    Category::Syntax | Category::Data =>
                    {
                        warn!("{peer}: received invalid data with error: {e}");
                    }
                },
            }
        }
    }

    fn listen_communication(self: Arc<Self>)
    {
        let listener = TcpListener::bind(SocketAddrV4::new([0, 0, 0, 0].into(), PORT)).unwrap();
        let pool = ThreadPool::new(self.cfg.lock().unwrap().count_comm_workers);

        for stream in listener.incoming()
        {
            let stream = stream.unwrap();

            let cpy = Arc::clone(&self);

            pool.execute(move || {
                cpy.handle_connection(Connection::new(stream));
            });
        }
    }

    fn build_blockchain(&mut self)
    {
        info!("building blockchain from disk");

        let mut blks = vec![];

        if let Ok(it) = std::fs::read_dir(&self.cfg.lock().unwrap().blkpath)
        {
            for i in it
            {
                let contents = std::fs::read_to_string(i.unwrap().path()).unwrap();

                if let Ok(blk) = serde_json::from_str::<Block>(&contents)
                {
                    info!("processing block {}", blk.hash_str());
                    blks.push(blk);
                }
                else
                {
                    todo!();
                }
            }
        }
        else
        {
            todo!();
        }

        self.state.lock().unwrap().chain = Blockchain::try_from(blks).unwrap();
    }

    fn build_cache(self: Arc<Self>)
    {
        let mut guard = self.state.lock().unwrap();
        let state = guard.deref_mut();

        let chain = &state.chain;
        let cache = &mut state.economy;

        for (_, blk) in chain.get_blocks()
        {
            *cache.entry(blk.get_miner().clone()).or_insert(0) += 10;

            for trx in blk.transactions()
            {
                let inp = trx.input();
                let out = trx.output();

                *cache.get_mut(inp.get_addr()).expect("wtf??") -= inp.get_value();
                let mut change = inp.get_value();

                for actor in out.transactors()
                {
                    change -= actor.get_value();
                    *cache.entry(actor.get_addr().clone()).or_insert(0) += actor.get_value();
                }

                *cache.get_mut(blk.get_miner()).unwrap() += change;
            }
        }

        for (addr, money) in cache
        {
            println!("{}: {}", addr.hash_str(), money);
        }
    }

    pub fn start(mut self)
    {
        self.build_blockchain();

        let peers = {
            let state = self.state.lock().unwrap();
            state.peers.clone()
        };
        let arc = Arc::new(self);
        for i in peers
        {
            Node::register_to_peer(Arc::clone(&arc), i.address().clone());
        }

        if arc.cfg.lock().unwrap().build_cache
        {
            Node::build_cache(Arc::clone(&arc));
        }

        let arccomm = Arc::clone(&arc);
        let commu = std::thread::spawn(move || Node::listen_communication(arccomm));

        let arcmine = Arc::clone(&arc);
        let mine = std::thread::spawn(move || Node::mine(arcmine));

        commu.join().unwrap();
        mine.join().unwrap();
    }
}
