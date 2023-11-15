mod lps;

use std::any::Any;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use lps::core::{gpu::Gpu, cpu::Cpu, bus::Bus};
use lps::rasterize::vertex_shader::CustomVertexShader;
use lps::rasterize::pixel_shader::CustomPixelShader;
use crate::lps::common::color::Color;
use crate::lps::common::math::mat4x4::Mat4x4;
use crate::lps::common::math::vec2::Vec2;
use crate::lps::common::math::vec3::Vec3;
use crate::lps::common::math::vec4::Vec4;

use crate::lps::core::common::Unit;
use crate::lps::rasterize::render_cmds::draw::Draw;
use crate::lps::rasterize::render_cmds::set_constant_buffer::SetConstantBufferCmd;
use crate::lps::rasterize::render_cmds::set_render_target::SetRenderTargetCmd;
use crate::lps::rasterize::render_cmds::set_vertex_buffer::SetVertexBufferCmd;
use crate::lps::rasterize::render_target::RenderTarget;
use crate::lps::rasterize::vt_input::VertexShaderInput;
use crate::lps::rasterize::vt_output::VertexShaderOutput;

fn get_viewport_mat(ox: i32, oy: i32, width: i32, height: i32) -> Mat4x4 {
    let mut mat = Mat4x4::new_with_value(1.0);
    mat[0][0] = width as f32 / 2.0;
    mat[3][0] = ox as f32 + width as f32 / 2.0;
    mat[1][1] = height as f32 / 2.0;
    mat[3][1] = oy as f32 + height as f32 / 2.0;
    return mat;
}

fn do_render(cpu: &mut Cpu) {
    let v1: VertexShaderInput = VertexShaderInput::new(
        Vec4::new(-0.5, -0.5, 0.0, 1.0),
        Vec3::new(255.0, 0.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    let v2: VertexShaderInput = VertexShaderInput::new(
        Vec4::new(0.5, -0.5, 0.0, 1.0),
        Vec3::new(0.0, 255.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    let v3: VertexShaderInput = VertexShaderInput::new(
        Vec4::new(0.0, 0.5, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 255.0),
        Vec2::new(0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    let render_target = Arc::new(Mutex::new(RenderTarget::new(800, 600)));
    let vertex_list: Vec<Arc<dyn Any + Send + Sync>> = vec![
        Arc::new(v1),
        Arc::new(v2),
        Arc::new(v3)];

    let mut unwrap = render_target.lock().unwrap();
    unwrap.clear(Color::BLUE);
    drop(unwrap);

    cpu.add_cmd(Box::new(SetConstantBufferCmd::new_with_mat4x4(
        0,
        Mat4x4::identity(),
    ))); // model matrix

    cpu.add_cmd(Box::new(SetConstantBufferCmd::new_with_mat4x4(
        1,
        get_viewport_mat(0, 0, 800, 600),
    ))); // view matrix

    cpu.add_cmd(Box::new(SetConstantBufferCmd::new_with_mat4x4(
        2,
        Mat4x4::identity(),
    ))); // proj matrix

    cpu.add_cmd(Box::new(SetRenderTargetCmd::new(Arc::clone(&render_target))));
    cpu.add_cmd(Box::new(SetVertexBufferCmd::new(vertex_list)));
    cpu.add_cmd(Box::new(Draw::new()));

    cpu.swap();

    // let mut unwrap = render_target.lock().unwrap();

    // unwrap.save("test.png");
}

fn main() {
    let bus = Arc::new(Mutex::new(Bus::new()));
    let condvar_info = Arc::new((Mutex::<i32>::new(2), Condvar::new()));

    let mut cpu = Cpu::new(&bus, &condvar_info);
    let mut gpu =
        Gpu::<VertexShaderInput, VertexShaderOutput>::new(&bus, &condvar_info);

    cpu.init();
    gpu.init();

    gpu.bind_vertex_shader(Box::new(CustomVertexShader::new()));
    gpu.bind_pixel_shader(Box::new(CustomPixelShader::new()));


    thread::scope(|scope| {
        let t1 = scope.spawn(|| {
            do_render(&mut cpu);
            cpu.start();
        });

        let t2 = scope.spawn(|| {
            gpu.start();
        });

        t1.join().unwrap();
        t2.join().unwrap();
    });

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
