use crate::lps::common::math::mat4x4::Mat4x4;
use crate::lps::common::math::vec4::Vec4;
use crate::lps::rasterize::render_cmds::clear::ClearCmd;
use crate::lps::rasterize::render_cmds::draw::DrawCmd;
use crate::lps::rasterize::render_cmds::render_cmd::RenderCmd;
use crate::lps::rasterize::render_cmds::set_constant_buffer::SetConstantBufferCmd;
use crate::lps::rasterize::render_cmds::set_render_target::SetRenderTargetCmd;
use crate::lps::rasterize::render_cmds::set_vertex_buffer::SetVertexBufferCmd;
use crate::lps::rasterize::render_target::RenderTarget;
use std::any::Any;
use std::sync::{Arc, Mutex};

use super::bus::{BusMutex, ExitNotifyCondVar, RenderCompleteNotifyCondVar};
use super::unit::Unit;

pub struct Cpu<'a> {
    bus_mutex: &'a BusMutex<'a>,
    exit_condvar: &'a ExitNotifyCondVar,
    exit_flag: Arc<Mutex<bool>>,
    render_complete_condvar: &'a RenderCompleteNotifyCondVar,
}

impl<'a> Cpu<'a> {
    pub fn new(
        bus_mutex: &'a BusMutex<'a>,
        condvar: &'a ExitNotifyCondVar,
        render_complete_condvar: &'a RenderCompleteNotifyCondVar,
    ) -> Cpu<'a> {
        Cpu {
            bus_mutex,
            exit_condvar: condvar,
            exit_flag: Arc::new(Mutex::new(true)),
            render_complete_condvar,
        }
    }

    pub fn add_cmd(&mut self, cmd: Box<dyn RenderCmd>) {
        let mut bus = self.bus_mutex.lock().unwrap();
        bus.add_cmd(cmd);
    }

    /// Let Cpu wait for Gpu compleate render.
    pub fn swap(&self) {
        let (mutex, condvar) = self.render_complete_condvar.as_ref();
        let guard = mutex.lock().unwrap();

        println!("cpu wait for render complete.");
        let _unused = condvar.wait(guard).unwrap();
        println!("cpu render complete.");
    }

    pub fn bind_vertex_buffer(&mut self, vertex_buffer: Vec<Arc<dyn Any + Send + Sync>>) {
        self.add_cmd(Box::new(SetVertexBufferCmd::new(vertex_buffer)));
    }

    pub fn bind_render_target(&mut self, render_target: Arc<Mutex<RenderTarget>>) {
        self.add_cmd(Box::new(SetRenderTargetCmd::new(render_target)));
    }

    pub fn bind_constant_buffer_mat4x4(&mut self, index: usize, mat: Mat4x4) {
        self.add_cmd(Box::new(SetConstantBufferCmd::new_with_mat4x4(index, mat)));
    }

    pub fn bind_constant_buffer_vec4(&mut self, index: usize, vec: Vec4) {
        self.add_cmd(Box::new(SetConstantBufferCmd::new_with_vec4(index, vec)));
    }

    pub fn clear(&mut self, color: Vec4) {
        self.add_cmd(Box::new(ClearCmd::new(color)));
    }

    pub fn draw(&mut self) {
        self.add_cmd(Box::new(DrawCmd::new()));
    }
}

impl<'a> Unit for Cpu<'a> {
    fn init(&mut self) {}

    fn start(&mut self) {
        println!("cpu start.");

        // let mut exit = self.exit_flag.as_ref().lock().unwrap();
        // *exit = false;
        //
        // loop {
        //     let exit = self.exit_flag.as_ref().lock().unwrap();
        //     if *exit {
        //         break;
        //     }
        //     drop(exit);
        // }

        let exit_condvar = ExitNotifyCondVar::clone(self.exit_condvar);

        let (lock, condvar) = exit_condvar.as_ref();
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
