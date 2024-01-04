mod lps;

use crate::lps::common::math::mat4x4::Mat4x4;
use crate::lps::common::math::vec2::Vec2;
use crate::lps::common::math::vec3::Vec3;
use crate::lps::common::math::vec4::Vec4;
use crate::lps::common::mesh::Mesh;
use crate::lps::common::render_window::RenderWindow;
use crate::lps::common::texture::Texture;
use lps::core::{bus::Bus, cpu::Cpu, gpu::Gpu};
use lps::rasterize::pixel_shader::CustomPixelShader;
use lps::rasterize::vertex_shader::CustomVertexShader;
use std::ops::DerefMut;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use crate::lps::core::unit::Unit;
use crate::lps::rasterize::render_target::RenderTarget;
use crate::lps::rasterize::vt_input::VertexShaderInput;
use crate::lps::rasterize::vt_output::VertexShaderOutput;

fn create_plane(
    left_top: &Vec3,
    left_bottom: &Vec3,
    right_bottom: &Vec3,
    right_top: &Vec3,
    normal: &Vec3,
) -> Mesh<VertexShaderInput> {
    #[rustfmt::skip]
    return Mesh::<VertexShaderInput>::new_with_data(vec![
        VertexShaderInput::new(Vec4::new(left_top.x, left_top.y, left_top.z, 1.0),
                               Vec3::new(255.0, 0.0, 0.0),
                               Vec2::new(0.0, 1.0),
                               normal.clone()),
        VertexShaderInput::new(Vec4::new(right_top.x, right_top.y, right_top.z, 1.0),
                               Vec3::new(0.0, 255.0, 0.0),
                               Vec2::new(1.0, 1.0),
                               normal.clone()),
        VertexShaderInput::new(Vec4::new(right_bottom.x, right_bottom.y, right_bottom.z, 1.0),
                               Vec3::new(0.0, 0.0, 255.0),
                               Vec2::new(1.0, 0.0),
                               normal.clone()),
        VertexShaderInput::new(Vec4::new(left_bottom.x, left_bottom.y, left_bottom.z, 1.0),
                               Vec3::new(255.0, 0.0, 255.0),
                               Vec2::new(0.0, 0.0),
                               normal.clone()),
    ], 
    vec![0, 2, 1, 0, 3, 2]);
}

fn create_box(center: &Vec3, radius: f32) -> Mesh<VertexShaderInput> {
    let mut result: Mesh<VertexShaderInput> = Mesh::new_with_data(vec![], vec![]);
    let front = create_plane(
        &(*center + Vec3::new(-radius, radius, radius)),
        &(*center + Vec3::new(-radius, -radius, radius)),
        &(*center + Vec3::new(radius, -radius, radius)),
        &(*center + Vec3::new(radius, radius, radius)),
        &Vec3::new(0.0, 0.0, 1.0),
    );
    result.add_mesh(&front);

    let left = create_plane(
        &(*center + Vec3::new(-radius, radius, -radius)),
        &(*center + Vec3::new(-radius, -radius, -radius)),
        &(*center + Vec3::new(-radius, -radius, radius)),
        &(*center + Vec3::new(-radius, radius, radius)),
        &Vec3::new(-1.0, 0.0, 0.0),
    );
    result.add_mesh(&left);

    let right = create_plane(
        &(*center + Vec3::new(radius, radius, radius)),
        &(*center + Vec3::new(radius, -radius, radius)),
        &(*center + Vec3::new(radius, -radius, -radius)),
        &(*center + Vec3::new(radius, radius, -radius)),
        &Vec3::new(1.0, 0.0, 0.0),
    );
    result.add_mesh(&right);

    let back = create_plane(
        &(*center + Vec3::new(radius, radius, -radius)),
        &(*center + Vec3::new(radius, -radius, -radius)),
        &(*center + Vec3::new(-radius, -radius, -radius)),
        &(*center + Vec3::new(-radius, radius, -radius)),
        &Vec3::new(0.0, 0.0, -1.0),
    );
    result.add_mesh(&back);

    let up = create_plane(
        &(*center + Vec3::new(-radius, radius, -radius)),
        &(*center + Vec3::new(-radius, radius, radius)),
        &(*center + Vec3::new(radius, radius, radius)),
        &(*center + Vec3::new(radius, radius, -radius)),
        &Vec3::new(0.0, 1.0, 0.0),
    );
    result.add_mesh(&up);

    let down = create_plane(
        &(*center + Vec3::new(-radius, -radius, radius)),
        &(*center + Vec3::new(-radius, -radius, -radius)),
        &(*center + Vec3::new(radius, -radius, -radius)),
        &(*center + Vec3::new(radius, -radius, radius)),
        &Vec3::new(0.0, -1.0, 0.0),
    );
    result.add_mesh(&down);

    return result;
}

fn create_triangle() -> Mesh<VertexShaderInput> {
    // let v1: VertexShaderInput = VertexShaderInput::new(
    //     Vec4::new(-0.5, -0.5, 0.0, 1.0),
    //     Vec3::new(255.0, 0.0, 0.0),
    //     Vec2::new(0.0, 0.0),
    //     Vec3::new(0.0, 0.0, 0.0),
    // );
    //
    // let v2: VertexShaderInput = VertexShaderInput::new(
    //     Vec4::new(0.5, -0.5, 0.0, 1.0),
    //     Vec3::new(0.0, 255.0, 0.0),
    //     Vec2::new(0.0, 0.0),
    //     Vec3::new(0.0, 0.0, 0.0),
    // );
    //
    // let v3: VertexShaderInput = VertexShaderInput::new(
    //     Vec4::new(0.0, 0.5, 0.0, 1.0),
    //     Vec3::new(0.0, 0.0, 255.0),
    //     Vec2::new(0.0, 0.0),
    //     Vec3::new(0.0, 0.0, 0.0),
    // );

    let v1: VertexShaderInput = VertexShaderInput::new(
        Vec4::new(-200.0 / 800.0, -150.0 / 600.0, 0.0, 1.0),
        Vec3::new(255.0, 0.0, 0.0), // r
        Vec2::new(0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    let v2: VertexShaderInput = VertexShaderInput::new(
        Vec4::new(-100.0 / 800.0, 300.0 / 600.0, 0.0, 1.0),
        Vec3::new(0.0, 255.0, 0.0), // g
        Vec2::new(0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    let v3: VertexShaderInput = VertexShaderInput::new(
        Vec4::new(300.0 / 800.0, 100.0 / 600.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 255.0), // b
        Vec2::new(0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    let vertex_list = vec![v1, v2, v3];
    let index_list = vec![0, 1, 2];

    Mesh::new_with_data(vertex_list, index_list)
}

fn do_render(cpu: &mut Cpu) {
    let mut window = RenderWindow::create("test".to_string(), 800, 600);
    window.init();

    let mesh = create_box(&Vec3::new(0.0, 0.0, 0.0), 0.5);
    let render_target = Arc::new(Mutex::new(RenderTarget::new(800, 600)));
    let texture = Arc::new(Mutex::new(Texture::load("./data/wall.jpg")));

    let mut angle = 0.0f32;
    let axis = Vec3::new(1.0, 1.0, 0.0).normal();

    cpu.bind_constant_buffer_mat4x4(
        1,
        Mat4x4::view_mat(
            &Vec3::new(0.0, 0.0, 5.0),
            &Vec3::new(0.0, 0.0, -1.0),
            &Vec3::new(1.0, 0.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
        ),
    ); // view matrix
    cpu.bind_constant_buffer_mat4x4(
        2,
        Mat4x4::perspective_mat(60.0f32.to_radians(), 800.0 / 600.0, 0.3, 100.0),
    ); // proj matrix

    cpu.bind_constant_buffer_texture(3, Arc::clone(&texture));

    cpu.bind_render_target(Arc::clone(&render_target));
    cpu.bind_mesh(&mesh);

    let mut i = 0;
    loop {
        let rotate = Mat4x4::rotate_axis_mat(angle.to_radians(), axis.clone());
        // cpu.bind_constant_buffer_mat4x4(0, Mat4x4::rotate_y_mat(45.0f32.to_radians())); // model matrix
        cpu.bind_constant_buffer_mat4x4(0, rotate); // model matrix
        cpu.clear(Vec4::new(0.0, 0.0, 0.0, 1.0));
        cpu.draw(true);
        cpu.swap();

        let mut unwrap = render_target.lock().unwrap();
        // let file_name = format!("output/output_{}.png", i);
        // unwrap.save(&file_name);

        let exit = window.update(unwrap.deref_mut());

        angle += 360.0 / 20.0;
        i += 1;

        if exit {
            break;
        }
    }
}

fn main() {
    let bus = Arc::new(Mutex::new(Bus::new()));
    let exit_condvar_info = Arc::new((Mutex::<i32>::new(2), Condvar::new()));
    let render_complete_condvar_info = Arc::new((Mutex::<i32>::new(0), Condvar::new()));

    let mut cpu = Cpu::new(&bus, &exit_condvar_info, &render_complete_condvar_info);
    let mut gpu = Gpu::<VertexShaderInput, VertexShaderOutput>::new(
        &bus,
        &exit_condvar_info,
        &render_complete_condvar_info,
    );

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

    let (lock, cvar) = exit_condvar_info.as_ref();
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
