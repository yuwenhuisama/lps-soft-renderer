use super::{
    bus::{BusMutex, ExitNotifyCondVar},
    common::Unit,
};
use crate::lps::rasterize::pipeline::{PipeLine, PixelShader, VertexShader};
use crate::lps::rasterize::render_target::RenderTarget;
use std::fmt::Debug;
use std::sync::Arc;
use std::thread::{self, Scope, ScopedJoinHandle};
use std::time::Duration;
use std::{any::Any, sync::Mutex};

pub trait GpuApi<'a> {
    fn set_vertex_buffer(&mut self, vertex_list: Vec<Arc<dyn Any + Send + Sync>>);
    fn set_render_target(&mut self, render_target: Arc<Mutex<RenderTarget>>);
    fn set_constant_buffer(&mut self, layout_index: usize, buffer: Arc<(dyn Any + Send + Sync)>);
    fn draw(&mut self);
}

pub struct Gpu<'a, VSInput, VSOutput> {
    bus_mutex: &'a BusMutex<'a>,
    pipe_line: PipeLine<VSInput, VSOutput>,
    exit_condvar: &'a ExitNotifyCondVar,
    vertex_list: Option<Vec<VSInput>>,
    render_target: Option<Arc<Mutex<RenderTarget>>>,
    constant_buffer: Vec<Option<Arc<dyn Any + Send>>>,
    exit_flag: Arc<Mutex<bool>>,
    scope_handler: Option<ScopedJoinHandle<'a, ()>>,
}

impl<'a, VSInput, VSOutput> Gpu<'a, VSInput, VSOutput> {
    pub fn new(
        bus_mutex: &'a BusMutex<'a>,
        condvar: &'a ExitNotifyCondVar,
    ) -> Gpu<'a, VSInput, VSOutput> {
        let mut constant_buffer = vec![];
        for _ in 0..32 {
            constant_buffer.push(None);
        }

        Gpu {
            bus_mutex,
            pipe_line: PipeLine::new(None, None),
            exit_condvar: condvar,
            vertex_list: None,
            render_target: None,
            constant_buffer, // 31 is the max layout index
            exit_flag: Arc::new(Mutex::new(true)),
            scope_handler: None,
        }
    }

    pub fn bind_vertex_shader(
        &mut self,
        vertex_shader: Box<dyn VertexShader<VSInput, VSOutput> + Send + Sync>,
    ) {
        self.pipe_line.bind_vertex_shader(Some(vertex_shader));
    }

    pub fn bind_pixel_shader(
        &mut self,
        pixel_shader: Box<dyn PixelShader<VSOutput> + Send + Sync>,
    ) {
        self.pipe_line.bind_pixel_shader(Some(pixel_shader));
    }
}

impl<'a, VSInput, VSOutput> GpuApi<'a> for Gpu<'a, VSInput, VSOutput>
where
    VSInput: 'a + Sync + Send + Debug + Copy + Clone + 'static,
    VSOutput: 'a + Sync + Send + Debug + Copy + Clone + 'static,
{
    fn set_vertex_buffer(&mut self, vertex_list: Vec<Arc<dyn Any + Send + Sync>>) {
        let vertex_list = vertex_list
            .iter()
            .map(|vertex| {
                let vt = vertex.downcast_ref::<VSInput>();
                if let None = vt {
                    panic!("vertex type is not matched");
                }
                *(vt.unwrap())
            })
            .collect::<Vec<VSInput>>();
        self.vertex_list = Some(vertex_list);
    }

    fn set_render_target(&mut self, render_target: Arc<Mutex<RenderTarget>>) {
        self.render_target = Some(render_target);
    }

    fn set_constant_buffer(&mut self, layout_index: usize, buffer: Arc<(dyn Any + Send + Sync)>) {
        self.constant_buffer[layout_index] = Some(buffer);
    }

    fn draw(&mut self) {
        if let None = self.vertex_list {
            panic!("vertex buffer is not set");
        }

        if let None = self.render_target {
            panic!("render target is not set");
        }

        let handled_vertex_list = self
            .vertex_list
            .as_ref()
            .unwrap()
            .iter()
            .map(|vertex| {
                self.pipe_line
                    .handle_vertex_shader(vertex, &self.constant_buffer)
            })
            .collect::<Vec<VSOutput>>();

        for vertex in handled_vertex_list {
            print!("handled vertex: {:?}", vertex);
        }
    }
}

unsafe impl<'a, VSInput, VSOutput> Sync for Gpu<'a, VSInput, VSOutput> {}

unsafe impl<'a, VSInput, VSOutput> Send for Gpu<'a, VSInput, VSOutput> {}

impl<'a, VSInput, VSOutput> Unit<'a> for Gpu<'a, VSInput, VSOutput>
where
    VSInput: 'static + Debug + Sync + Send + Copy + Clone,
    VSOutput: 'static + Debug + Sync + Send + Copy + Clone,
{
    fn init(&mut self) {}

    fn start(&mut self) {
        println!("gpu start.");

        let exit_condvar = Arc::clone(&self.exit_condvar);
        let bus_mutex = Arc::clone(&self.bus_mutex);
        let mut exit = self.exit_flag.as_ref().lock().unwrap();
        *exit = false;
        drop(exit);

        loop {
            let exit = self.exit_flag.as_ref().lock().unwrap();
            if *exit {
                break;
            }
            drop(exit);

            let mut bus = bus_mutex.lock().unwrap();

            if bus.empty() {
                thread::sleep(Duration::from_millis(1));
            }

            let result = bus.try_get_cmd();
            let cmd = match result {
                Result::Ok(res) => res,
                Err(_) => continue,
            };

            print!("gpu get cmd: {:?}\n", cmd.as_ref().cmd_type());
            cmd.execute(self);
        }

        let (lock, condvar) = exit_condvar.as_ref();
        let mut cnt = lock.lock().unwrap();
        *cnt -= 1;
        condvar.notify_all();

        println!("gpu exit.")
    }

    fn exit(&mut self) {
        let mut exit = self.exit_flag.as_ref().lock().unwrap();
        *exit = true;
    }
}
