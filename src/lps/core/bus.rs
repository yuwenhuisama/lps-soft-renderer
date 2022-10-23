use std::{collections::VecDeque, sync::{Arc, Mutex, Condvar}};
use super::commands::{RenderCommand};

pub enum BusPopRes {
    Ok,
    Failed,
}

pub struct Bus {
    cmd_queue_: VecDeque<Box<dyn RenderCommand>>
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            cmd_queue_: VecDeque::<Box<dyn RenderCommand>>::new()
        }
    }

    pub fn add_cmd(&mut self, cmd: Box<dyn RenderCommand>) {
        self.cmd_queue_.push_back(cmd);
    }

    pub fn try_get_cmd(&mut self) -> Result<Box<dyn RenderCommand>, BusPopRes> {
        if let Some(res) = self.cmd_queue_.pop_front() {
            Ok(res)
        }
        else {
            Err(BusPopRes::Failed)
        }
    }
}

pub type BusMutex = Arc<Mutex<Bus>>;
pub type ExitNotifyCondVar = Arc<(Mutex<i32>, Condvar)>;
