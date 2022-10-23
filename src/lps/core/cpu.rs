use std::thread;

use super::bus::{Bus, BusMutex, ExitNotifyCondVar};
use super::common::{Unit};

pub struct Cpu<'a> {
    cmd_bus: &'a BusMutex,
    exit_condvar: &'a ExitNotifyCondVar,
}

impl<'a> Cpu<'a> {
    pub fn new(bus: &'a BusMutex, condvar: &'a ExitNotifyCondVar) -> Cpu<'a> {
        Cpu {
            cmd_bus: bus,
            exit_condvar: condvar,
        }
    }
}

impl<'a> Unit for Cpu<'a> {
    fn init(&mut self) {
    }

    fn start(&mut self) {
        // let cmd_bus = BusMutex::clone(self.cmd_bus);
        let exit_condivar = ExitNotifyCondVar::clone(self.exit_condvar);

        thread::spawn(move || {
            let (lock, condvar) = exit_condivar.as_ref();
            let mut cnt = lock.lock().unwrap();
            *cnt -= 1;
            condvar.notify_all();

            println!("cpu exit.")
        });
    }

    fn exit(&mut self) {
    }
}
