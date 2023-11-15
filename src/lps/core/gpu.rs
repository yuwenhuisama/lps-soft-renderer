use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use std::thread;
use crate::lps::rasterize::pipeline::PipeLine;
use crate::lps::rasterize::render_target::RenderTarget;
use super::{bus::{BusMutex, ExitNotifyCondVar}, common::Unit};

pub trait GpuApi<'a> {
    fn set_vertex_buffer(&mut self, vertex_list: Vec<Arc<dyn Any + Send + Sync + 'a>>);
    fn set_render_target(&mut self, render_target: &'a RenderTarget);
    fn set_constant_buffer(&mut self, layout_index: usize, buffer: &'a (dyn Any + Send + Sync));
    fn draw(&mut self);
}

pub struct Gpu<'a, VSInput, VSOutput>
{
    bus_mutex: &'a BusMutex<'a> ,
    pipe_line: PipeLine<'a, VSInput, VSOutput>,
    exit_condvar: &'a ExitNotifyCondVar,
    vertex_list: Option<Vec<VSInput>>,
    render_target: Option<&'a RenderTarget>,
    constant_buffer: Vec<Option<Box<dyn 'static + Any + Send>>>,
}

impl<'a, VSInput, VSOutput> Gpu<'a, VSInput, VSOutput> {
    pub fn new(bus_mutex: &'a BusMutex<'a>, condvar: &'a ExitNotifyCondVar) -> Gpu<'a, VSInput, VSOutput> {
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
        }
    }
}

impl<'a, VSInput, VSOutput> GpuApi<'a> for Gpu<'a, VSInput, VSOutput>
    where VSInput: 'a + Sync + Send + Debug + Copy + Clone + 'static,
          VSOutput: 'a + Sync + Send + Debug + Copy + Clone + 'static,
{
    fn set_vertex_buffer(&mut self, vertex_list: Vec<Arc<dyn Any + Send + Sync>>) {
        let vertex_list = vertex_list.iter().map(|vertex| {
            let vt = vertex.downcast_ref::<VSInput>();
            if let None = vt {
                panic!("vertex type is not matched");
            }
            *(vt.unwrap())
        }).collect::<Vec<VSInput>>();
        self.vertex_list = Some(vertex_list);
    }

    fn set_render_target(&mut self, render_target: &'a RenderTarget) {
        self.render_target = Some(render_target);
    }

    fn set_constant_buffer(&mut self, layout_index: usize, buffer: &'a (dyn Any + Send + Sync)) {
        self.constant_buffer[layout_index] = Some(Box::new(buffer.clone()));
    }

    fn draw(&mut self) {
        if let None = self.vertex_list {
            panic!("vertex buffer is not set");
        }

        if let None = self.render_target {
            panic!("render target is not set");
        }

        let handled_vertex_list =
            self.vertex_list.as_ref().unwrap().iter().map(|vertex| {
                self.pipe_line.handle_vertex_shader(vertex, &self.constant_buffer)
            }).collect::<Vec<VSOutput>>();

        for vertex in handled_vertex_list {
            print!("handled vertex: {:?}", vertex);
        }
    }
}

impl<'a, VSInput, VSOutput> Unit for Gpu<'a, VSInput, VSOutput>
    where VSInput: 'a + Debug + Sync + Send + Copy + Clone,
          VSOutput: 'a + Debug + Sync + Send + Copy + Clone,
{
    fn init(&mut self) {}

    fn start(&mut self) {
        let exit_condvar = ExitNotifyCondVar::clone(self.exit_condvar);

        thread::spawn(|| {
            loop {
                let bus = self.bus_mutex.lock().as_mut().unwrap();

                if bus.empty() {
                    break;
                }

                let result = bus.try_get_cmd();
                let cmd = match result {
                    Result::Ok(res) => res,
                    Err(_) => continue,
                };

                cmd.execute(self);
            }
        });

        let (lock, condvar) = exit_condvar.as_ref();
        let mut cnt = lock.lock().unwrap();
        *cnt -= 1;
        condvar.notify_all();

        println!("gpu exit.")
    }

    fn exit(&mut self) {
        todo!()
    }
}

