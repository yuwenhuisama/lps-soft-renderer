use std::thread;
use crate::lps::rasterize::render_cmds::render_cmd::RenderCmd;

use super::bus::{BusMutex, ExitNotifyCondVar};
use super::common::{Unit};

pub struct Cpu<'a> {
    bus_mutex: &'a BusMutex<'a> ,
    exit_condvar: &'a ExitNotifyCondVar,
}

impl<'a> Cpu<'a> {
    pub fn new(bus_mutex: &'a BusMutex<'a>, condvar: &'a ExitNotifyCondVar) -> Cpu<'a> {
        Cpu {
            bus_mutex,
            exit_condvar: condvar,
        }
    }

    pub fn add_cmd(&mut self, cmd: Box<dyn RenderCmd<'a>>) {
        let mut bus = self.bus_mutex.lock().unwrap();
        bus.add_cmd(cmd);
    }
}

impl<'a> Unit for Cpu<'a> {
    fn init(&mut self) {}

    fn start(&mut self) {
        let exit_condivar = ExitNotifyCondVar::clone(self.exit_condvar);

        thread::spawn(move || {
            let (lock, condvar) = exit_condivar.as_ref();
            let mut cnt = lock.lock().unwrap();
            *cnt -= 1;
            condvar.notify_all();

            println!("cpu exit.")
        });
    }

    fn exit(&mut self) {}
}
