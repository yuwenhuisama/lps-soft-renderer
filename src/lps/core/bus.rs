use std::{collections::VecDeque, sync::{Arc, Mutex, Condvar}};
use crate::lps::rasterize::render_cmds::render_cmd::RenderCmd;

pub enum BusPopRes {
    Succ,
    Failed,
}

pub struct Bus<'a> {
    cmd_queue_: VecDeque<Box<dyn RenderCmd<'a>>>
}

impl<'a>  Bus<'a>  {
    pub fn new() -> Bus<'a>  {
        Bus {
            cmd_queue_: VecDeque::<Box<dyn RenderCmd<'a> >>::new()
        }
    }

    pub fn add_cmd(&mut self, cmd: Box<dyn RenderCmd<'a> >) {
        self.cmd_queue_.push_back(cmd);
    }

    pub fn try_get_cmd(&mut self) -> Result<Box<dyn RenderCmd<'a> >, BusPopRes> {
        if let Some(res) = self.cmd_queue_.pop_front() {
            Ok(res)
        }
        else {
            Err(BusPopRes::Failed)
        }
    }

    pub fn empty(&self) -> bool {
        self.cmd_queue_.is_empty()
    }
}

pub type BusMutex<'a>  = Arc<Mutex<Bus<'a> >>;
pub type ExitNotifyCondVar = Arc<(Mutex<i32>, Condvar)>;
