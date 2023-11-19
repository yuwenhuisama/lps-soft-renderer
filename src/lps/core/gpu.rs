use super::{
    bus::{BusMutex, ExitNotifyCondVar},
    unit::Unit,
};
use crate::lps::common::color::Color;
use crate::lps::common::math::mat4x4::Mat4x4;
use crate::lps::common::math::vec4::Vec4;
use crate::lps::core::bus::RenderCompleteNotifyCondVar;
use crate::lps::rasterize::pipeline::{PipeLine, PixelShader, VertexShader};
use crate::lps::rasterize::render_target::RenderTarget;
use crate::lps::rasterize::render_util::RenderUtil;
use crate::lps::rasterize::vt_output::VertexShaderOutputPositionAndLerp;
use std::fmt::Debug;
use std::ops::DerefMut;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::{any::Any, sync::Mutex};

pub trait GpuApi<'a> {
    fn set_vertex_buffer(&mut self, vertex_list: Vec<Arc<dyn Any + Send + Sync>>);
    fn set_index_buffer(&mut self, index_list: Vec<usize>);
    fn set_render_target(&mut self, render_target: Arc<Mutex<RenderTarget>>);
    fn set_constant_buffer(&mut self, layout_index: usize, buffer: Arc<(dyn Any + Send + Sync)>);
    fn draw(&mut self, draw_with_index: bool);
    fn clear(&self, color: &Vec4);
    fn swap(&mut self);
}

pub struct Gpu<'a, VSInput, VSOutput> {
    bus_mutex: &'a BusMutex<'a>,
    pipe_line: PipeLine<VSInput, VSOutput>,
    exit_condvar: &'a ExitNotifyCondVar,
    vertex_list: Option<Vec<VSInput>>,
    index_list: Option<Vec<usize>>,
    render_target: Option<Arc<Mutex<RenderTarget>>>,
    constant_buffer: Vec<Option<Arc<dyn Any + Send>>>,
    exit_flag: Arc<Mutex<bool>>,
    render_complete_condvar: &'a RenderCompleteNotifyCondVar,
    render_cnt: i32,
}

impl<'a, VSInput, VSOutput> Gpu<'a, VSInput, VSOutput> {
    pub fn new(
        bus_mutex: &'a BusMutex<'a>,
        condvar: &'a ExitNotifyCondVar,
        render_complete_condvar: &'a RenderCompleteNotifyCondVar,
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
            index_list: None,
            render_target: None,
            constant_buffer, // 31 is the max constant buffer index
            exit_flag: Arc::new(Mutex::new(true)),
            render_complete_condvar,
            render_cnt: 0,
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
    VSInput: 'static + Sync + Send + Debug + Copy + Clone,
    VSOutput: 'static + VertexShaderOutputPositionAndLerp + Sync + Send + Debug + Copy + Clone,
{
    fn set_vertex_buffer(&mut self, vertex_list: Vec<Arc<dyn Any + Send + Sync>>) {
        let vertex_list = vertex_list
            .iter()
            .map(|vertex| {
                let vt = vertex.downcast_ref::<VSInput>();
                if let None = vt {
                    panic!("vertex type is not matched: {:?}", Any::type_id(vertex));
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

    fn swap(&mut self) {
        self.render_cnt += 1;

        let (mutex, condvar) = self.render_complete_condvar.as_ref();
        let mut guard = mutex.lock().unwrap();
        *guard = self.render_cnt;

        condvar.notify_all();
    }

    fn draw(&mut self, draw_with_index: bool) {
        if let None = self.vertex_list {
            panic!("vertex buffer is not set");
        }

        if let None = self.render_target {
            panic!("render target is not set");
        }

        let mut render_target = self.render_target.as_ref().unwrap().lock().unwrap();
        let viewport_mat = Mat4x4::viewport_mat(
            0,
            0,
            render_target.width() as i32,
            render_target.height() as i32,
        );

        let pipe_line = &mut self.pipe_line;
        let handled_vertex_list = self
            .vertex_list
            .as_ref()
            .unwrap()
            .iter()
            .map(|vertex| {
                let mut vt = pipe_line.handle_vertex_shader(vertex, &self.constant_buffer);

                // apply perspective division
                let mut v = vt.position_as_mut().clone();
                v /= v.z;
                v.w = 1.0;
                v.z = (v.z + 1.0) / 2.0;

                // apply viewport transform
                v = viewport_mat * v;
                *(vt.position_as_mut()) = v;

                println!("vt: {:?}", vt);

                return vt;
            })
            .collect::<Vec<VSOutput>>();

        let vertex_cnt = if draw_with_index {
            self.index_list.as_ref().unwrap().len()
        } else {
            self.vertex_list.as_ref().unwrap().len()
        };

        let triangles = vertex_cnt / 3;

        if !draw_with_index {
            for i in 0..triangles {
                let v0 = &handled_vertex_list[i * 3];
                let v1 = &handled_vertex_list[i * 3 + 1];
                let v2 = &handled_vertex_list[i * 3 + 2];

                RenderUtil::draw_triangle(
                    render_target.deref_mut(),
                    v0,
                    v1,
                    v2,
                    |v0: &VSOutput, v1: &VSOutput, weight: f32| {
                        let color = VSOutput::lerp(v0, v1, weight);
                        let output = pipe_line.handle_pixel_shader(&color);
                        return output;
                    },
                );
            }
        } else {
            let index_list = self.index_list.as_ref().unwrap();
            for i in 0..triangles {
                let v0 = &handled_vertex_list[index_list[i * 3]];
                let v1 = &handled_vertex_list[index_list[i * 3 + 1]];
                let v2 = &handled_vertex_list[index_list[i * 3 + 2]];

                RenderUtil::draw_triangle(
                    render_target.deref_mut(),
                    v0,
                    v1,
                    v2,
                    |v0: &VSOutput, v1: &VSOutput, weight: f32| {
                        let color = VSOutput::lerp(v0, v1, weight);
                        let output = pipe_line.handle_pixel_shader(&color);
                        return output;
                    },
                );
            }
        }
    }

    fn clear(&self, color: &Vec4) {
        if let None = self.render_target {
            panic!("render target is not set");
        }

        let mut unwrap = self.render_target.as_ref().unwrap().lock().unwrap();
        unwrap.clear(Color::new_rgba(
            color.x as u8,
            color.y as u8,
            color.z as u8,
            color.w as u8,
        ));
    }

    fn set_index_buffer(&mut self, index_list: Vec<usize>) {
        self.index_list = Some(index_list);
    }
}

unsafe impl<'a, VSInput, VSOutput> Sync for Gpu<'a, VSInput, VSOutput> {}

unsafe impl<'a, VSInput, VSOutput> Send for Gpu<'a, VSInput, VSOutput> {}

impl<'a, VSInput, VSOutput> Unit for Gpu<'a, VSInput, VSOutput>
where
    VSInput: 'static + Debug + Sync + Send + Copy + Clone,
    VSOutput: 'static + VertexShaderOutputPositionAndLerp + Debug + Sync + Send + Copy + Clone,
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
                Ok(res) => res,
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
