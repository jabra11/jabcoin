use crate::network::{Connection, Peer};
use crate::node::Node;
use crate::threadpool::ThreadPool;
use jabcoin::{
    core::{Block, Transaction},
    network::{Header, Message},
};
use log::{debug, error, info, trace, warn};
use serde_json::error::Category;
use std::collections::VecDeque;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::sync::{
    atomic::{AtomicBool, Ordering::Relaxed},
    Arc, Condvar, Mutex, Weak,
};
use std::time::{Duration, Instant};

const PORT: u16 = 27182;

#[derive(Debug)]
pub struct Job
{
    schedule: Option<(Instant, Duration)>,
    peer_addr: Ipv4Addr,
    msg: Message,
}

impl Job
{
    pub fn new(peer_addr: Ipv4Addr, msg: Message) -> Job
    {
        Job {
            schedule: None,
            peer_addr,
            msg,
        }
    }

    pub fn _with_schedule(schedule: (Instant, Duration), peer_addr: Ipv4Addr, msg: Message) -> Job
    {
        Job {
            schedule: Some(schedule),
            peer_addr,
            msg,
        }
    }
}

#[derive(Debug)]
enum ComMessage
{
    Work(Job),
    Terminate,
}

#[derive(PartialEq, Eq)]
enum Status
{
    Running,
    ShuttingDown,
    Shutdown,
}

pub struct Communication
{
    node: Weak<Node>,
    work_queue: Mutex<VecDeque<ComMessage>>,
    cvar: Arc<Condvar>,
    shutdown: AtomicBool,
    status: Mutex<Status>,
}

impl Communication
{
    pub fn new(node: Weak<Node>) -> Communication
    {
        Communication {
            node,
            work_queue: Mutex::new(VecDeque::new()),
            cvar: Arc::new(Condvar::new()),
            shutdown: AtomicBool::new(false),
            status: Mutex::new(Status::Running),
        }
    }

    pub fn start(self: Arc<Self>)
    {
        let listener_arc = Arc::clone(&self);
        let listener_thd = std::thread::spawn(move || {
            listener_arc.listen_communication();
        });

        let tp = ThreadPool::new(3);

        let execute = |job: Job| match Connection::new_try_peer_addr(job.peer_addr, PORT)
        {
            Ok(mut conn) =>
            {
                if let Err(e) = conn.write_msg(&job.msg)
                {
                    warn!(
                        "failed to write message to {} with error {e}.",
                        job.peer_addr
                    );
                }
                else
                {
                    debug!(
                        "wrote message with header {:?} to {}.",
                        job.msg.header, job.peer_addr
                    );
                }
            }
            Err(e) =>
            {
                warn!("failed to connect to {} with error {e}.", job.peer_addr);
            }
        };

        while !Arc::clone(&self).shutdown.load(Relaxed)
        {
            trace!("waiting on cvar.");
            {
                // scope to kill lockguard, we just do this for the cvar wait
                let _q_guard = self.cvar.wait(self.work_queue.lock().unwrap()).unwrap();
            }
            trace!("woke up.");

            let cp = Arc::clone(&self);
            tp.execute(move || {
                while let Some(cmsg) = cp.work_queue.lock().unwrap().pop_back()
                {
                    match cmsg
                    {
                        ComMessage::Work(job) =>
                        {
                            if let Some((instant, duration)) = job.schedule
                            {
                                if instant.elapsed() > duration
                                {
                                    execute(job);
                                }
                                else
                                {
                                    cp.queue_job(job);
                                }
                            }
                            else
                            {
                                execute(job);
                            }
                        }
                        ComMessage::Terminate =>
                        {
                            cp.shutdown.store(true, Relaxed);
                            *cp.status.lock().unwrap() = Status::ShuttingDown;
                        }
                    }
                }
            });
        }

        debug!("trying to shut down listener thread ..");
        self.shutdown.store(true, Relaxed);

        match Connection::new_try_peer_addr(Ipv4Addr::new(127, 0, 0, 1), PORT)
        {
            // result doesn't matter, either way the listener should have received the shutdown
            // signal
            _ => (),
        }

        listener_thd.join().unwrap();
        debug!("listener thread shutdown.");
        *self.status.lock().unwrap() = Status::Shutdown;
    }

    pub fn register_to_peer(&self, peer: Ipv4Addr) -> Result<(), ()>
    {
        info!("trying to register to {peer}.");
        let slf_str = {
            let state_grd = self.node.upgrade().unwrap();
            let state = state_grd.state.lock().unwrap();
            serde_json::to_string(&state.peer).unwrap()
        };

        let msg = Message::with_data(Header::Register, &slf_str);
        if let Ok(mut conn) = Connection::new_try_peer_addr(peer, PORT)
        {
            if let Ok(_) = conn.write_msg(&msg)
            {
                info!("registered to {peer}.");
                Ok(())
            }
            else
            {
                warn!("failed to write message to {peer}.");
                Err(())
            }
        }
        else
        {
            warn!("failed to connect to {peer}.");
            Err(())
        }
    }

    pub fn request_peers(&self, peer_addr: Ipv4Addr)
    {
        info!("requesting peers from {peer_addr}.");
        let slf_str = {
            let state_grd = self.node.upgrade().unwrap();
            let state = state_grd.state.lock().unwrap();
            serde_json::to_string(&state.peer).unwrap()
        };

        let msg = Message::with_data(Header::RequestPeers, &slf_str);
        self.queue_job(Job::new(peer_addr, msg));
    }

    fn parse_msg(&self, msg: Message, peer_addr: &Ipv4Addr)
    {
        let node = &self.node.upgrade().unwrap();
        match msg.header
        {
            Header::Register =>
            {
                info!("{peer_addr}: received a register request.");
                let peer = match serde_json::from_str::<Peer>(&msg.body)
                {
                    Ok(p) => Some(p),
                    Err(e) =>
                    {
                        warn!(
                            "{peer_addr}: {:<30} {e}.",
                            "failed to parse request with error"
                        );
                        None
                    }
                };

                if let Some(mut peer) = peer
                {
                    let mut state = node.state.lock().unwrap();
                    peer.set_address(peer_addr.clone());
                    if !state.peers.contains(&peer)
                    {
                        let peers_str = serde_json::to_string(&state.peers).unwrap();

                        state.peers.push(peer);

                        info!("registered {peer_addr}.");
                        let msg = Message::with_data(Header::BroadcastPeers, &peers_str);
                        info!("sharing peers with {peer_addr}.");

                        self.queue_job(Job::new(*peer_addr, msg));
                    }
                    else
                    {
                        info!("{peer_addr}: already registered.");
                    }
                }
            }
            Header::BroadcastPeers =>
            {
                info!("{peer_addr}: received new peers.");
                let mut new_peers = serde_json::from_str::<Vec<Peer>>(&msg.body).unwrap();

                {
                    // only hold the mutex lock in this scope
                    let state = node.state.lock().unwrap();
                    new_peers = new_peers
                        .into_iter()
                        .filter(|p| !state.peers.contains(p))
                        .collect();
                }

                let mut good_peers = vec![];

                for i in new_peers
                {
                    if let Ok(_) = self.register_to_peer(i.address().clone())
                    {
                        self.request_peers(*i.address());
                        good_peers.push(i);
                    }
                }

                let mut state = node.state.lock().unwrap();

                for i in good_peers
                {
                    state.peers.push(i);
                }
            }
            Header::RequestPeers =>
            {
                info!("{peer_addr}: received request nodes request.");

                let state = node.state.lock().unwrap();
                let peers = serde_json::to_string::<Vec<Peer>>(&state.peers).unwrap();

                let msg = Message::with_data(Header::BroadcastPeers, &peers);
                self.queue_job(Job::new(*peer_addr, msg));
            }
            Header::Deregister =>
            {
                info!("{peer_addr}: received deregister request.");

                let mut state = node.state.lock().unwrap();

                let idx = state.peers.iter().position(|p| p.address() == peer_addr);

                if let Some(idx) = idx
                {
                    state.peers.swap_remove(idx);
                    info!("removed peer {peer_addr}.");
                }
                else
                {
                    warn!("Did not find {peer_addr} in memory.");
                }
            }
            Header::BroadcastTransaction =>
            {
                match serde_json::from_str::<Transaction>(&msg.body)
                {
                    Ok(t) =>
                    {
                        self.node.upgrade().unwrap().handle_new_transaction(t);
                    }
                    Err(e) =>
                    {
                        warn!("{peer_addr}: {:<30} {e}.", "failed to parse trx with error");
                    }
                };
            }
            Header::BroadcastBlock =>
            {
                match serde_json::from_str::<Block>(&msg.body)
                {
                    Ok(b) =>
                    {
                        node.handle_new_block(b, peer_addr);
                    }
                    Err(e) =>
                    {
                        warn!(
                            "{peer_addr}: {:<30} {e}.",
                            "failed to parse block with error"
                        );
                    }
                };
            }
            _ => todo!(),
        }
    }

    fn handle_connection(&self, mut conn: Connection)
    {
        let peer = conn.get_peer_addr();
        debug!("{peer}: new connection.");

        loop
        {
            // check msg validity
            match conn.read_msg()
            {
                Ok(m) => self.parse_msg(m, &peer),
                Err(e) => match e.classify()
                {
                    Category::Eof =>
                    {
                        break;
                    }
                    Category::Io =>
                    {
                        error!("{peer}: failed to read msg with i/o error: {e}.");
                    }
                    Category::Syntax | Category::Data =>
                    {
                        warn!("{peer}: received invalid data with error: {e}.");
                    }
                },
            }
        }
    }

    pub fn listen_communication(self: Arc<Self>)
    {
        let listener = TcpListener::bind(SocketAddrV4::new([0, 0, 0, 0].into(), PORT)).unwrap();
        let pool = ThreadPool::new(2);

        for stream in listener.incoming()
        {
            if self.shutdown.load(Relaxed) == true
            {
                debug!("shutting down listener.");
                break;
            }

            let stream = stream.unwrap();
            let cpy = Arc::clone(&self);

            pool.execute(move || {
                cpy.handle_connection(Connection::new(stream));
            });
        }
    }

    pub fn queue_job(&self, job: Job)
    {
        self.work_queue
            .lock()
            .unwrap()
            .push_back(ComMessage::Work(job));
        self.cvar.notify_one();
    }

    pub fn request_stop(&self)
    {
        self.work_queue.lock().unwrap().clear();

        for peer in &self.node.upgrade().unwrap().state.lock().unwrap().peers
        {
            let msg = Message::with_data(Header::Deregister, "");
            let job = Job::new(*peer.address(), msg);
            self.queue_job(job);
        }

        self.work_queue
            .lock()
            .unwrap()
            .push_back(ComMessage::Terminate);
        self.cvar.notify_one();

        while *self.status.lock().unwrap() != Status::Shutdown
        {
            self.cvar.notify_one();
        }
    }
}

impl Drop for Communication
{
    fn drop(&mut self)
    {
        if *self.status.lock().unwrap() == Status::Running
        {
            self.request_stop();
        }
        debug!("shutdown communication thread.");
    }
}
