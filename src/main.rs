mod lps;

use std::sync::{Arc, Mutex, Condvar};
use lps::core::{gpu::Gpu, cpu::Cpu, bus::Bus};

use crate::lps::core::common::Unit;

fn do_render() {

}

fn main() {
    println!("Start run");
    
    let bus = Arc::new(Mutex::new(Bus::new()));
    let condvar_info = Arc::new((Mutex::<i32>::new(2), Condvar::new()));

    let mut cpu = Cpu::new(&bus, &condvar_info);
    let mut gpu = Gpu::new(&bus, &condvar_info);

    cpu.init();
    gpu.init();

    cpu.start();
    gpu.start();

    do_render();

    let (lock, cvar) = condvar_info.as_ref();
    let mut cnt = lock.lock().unwrap();
    while *cnt > 0 {
        println!("waiting.");
        cnt = cvar.wait(cnt).unwrap();
        println!("waiting exit.");
    }

    cpu.exit();
    gpu.exit();

    print!("End run");
}
