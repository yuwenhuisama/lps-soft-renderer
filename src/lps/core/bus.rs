use crate::lps::rasterize::render_cmds::render_cmd::RenderCmd;
use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

pub enum BusPopRes {
    Succ,
    Failed,
}

pub struct Bus {
    cmd_queue_: VecDeque<Box<dyn RenderCmd>>,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            cmd_queue_: VecDeque::<Box<dyn RenderCmd>>::new(),
        }
    }

    pub fn add_cmd(&mut self, cmd: Box<dyn RenderCmd>) {
        self.cmd_queue_.push_back(cmd);
    }

    pub fn try_get_cmd(&mut self) -> Result<Box<dyn RenderCmd>, BusPopRes> {
        if let Some(res) = self.cmd_queue_.pop_front() {
            Ok(res)
        } else {
            Err(BusPopRes::Failed)
        }
    }

    pub fn empty(&self) -> bool {
        self.cmd_queue_.is_empty()
    }
}

pub type BusMutex<'a> = Arc<Mutex<Bus>>;
pub type ExitNotifyCondVar = Arc<(Mutex<i32>, Condvar)>;
pub type RenderCompleteNotifyCondVar = Arc<(Mutex<i32>, Condvar)>;
