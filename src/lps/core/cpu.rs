use crate::lps::rasterize::render_cmds::render_cmd::RenderCmd;
use std::sync::{Arc, Mutex};
use std::thread::{self, Scope};

use super::bus::{BusMutex, ExitNotifyCondVar};
use super::common::Unit;

pub struct Cpu<'a> {
    bus_mutex: &'a BusMutex<'a>,
    exit_condvar: &'a ExitNotifyCondVar,
    exit_flag: Arc<Mutex<bool>>,
}

impl<'a> Cpu<'a> {
    pub fn new(bus_mutex: &'a BusMutex<'a>, condvar: &'a ExitNotifyCondVar) -> Cpu<'a> {
        Cpu {
            bus_mutex,
            exit_condvar: condvar,
            exit_flag: Arc::new(Mutex::new(true)),
        }
    }

    pub fn add_cmd(&mut self, cmd: Box<dyn RenderCmd>) {
        let mut bus = self.bus_mutex.lock().unwrap();
        bus.add_cmd(cmd);
    }

    /// Let Cpu wait for Gpu compleate render.
    pub fn swap(&self) {}
}

impl<'a> Unit<'a> for Cpu<'a> {
    fn init(&mut self) {}

    fn start(&mut self) {
        println!("cpu start.");

        let mut exit = self.exit_flag.as_ref().lock().unwrap();
        *exit = false;

        loop {
            let exit = self.exit_flag.as_ref().lock().unwrap();
            if *exit {
                break;
            }
            drop(exit);
        }

        let exit_condivar = ExitNotifyCondVar::clone(self.exit_condvar);

        let (lock, condvar) = exit_condivar.as_ref();
        let mut cnt = lock.lock().unwrap();
        *cnt -= 1;
        condvar.notify_all();

        println!("cpu exit.")
    }

    fn exit(&mut self) {
        let mut exit = self.exit_flag.as_ref().lock().unwrap();
        *exit = true;
    }
}
