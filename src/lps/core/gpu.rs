use std::thread;
use super::{bus::{Bus, BusMutex, ExitNotifyCondVar}, common::Unit};

pub struct Gpu<'a> {
    cmd_bus: &'a BusMutex,
    exit_condvar : &'a ExitNotifyCondVar,
}

impl<'a> Gpu<'a> {
    pub fn new(bus: &'a BusMutex, condvar: &'a ExitNotifyCondVar) -> Gpu<'a> {
        Gpu {
            cmd_bus: bus,
            exit_condvar: condvar,
        }
    }
}

impl<'a> Unit for Gpu<'a> {
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

            println!("gpu exit.")
        });
    }

    fn exit(&mut self) {
    }
}
