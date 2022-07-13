pub mod network;
pub mod node;
pub mod threadpool;
pub mod wallet;

use std::sync::{Condvar, Mutex};

pub struct KillToken
{
    cvar: Condvar,
    kill: Mutex<bool>,
}

impl KillToken
{
    pub fn new() -> KillToken
    {
        KillToken {
            cvar: Condvar::new(),
            kill: Mutex::new(false),
        }
    }

    pub fn wait_on(&self) -> bool
    {
        match self.cvar.wait(self.kill.lock().unwrap())
        {
            Ok(b) => *b,
            Err(e) => panic!("{e}"),
        }
    }

    pub fn activate(&self)
    {
        *self.kill.lock().unwrap() = true;
        self.cvar.notify_one();
    }
}
