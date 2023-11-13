use crate::lps::common::math::{vec2::Vec2, vec3::Vec3, vec4::Vec4};

#[derive(Copy, Clone)]
pub struct VertexShaderOutput {
    world_pos: Vec4,
    window_pos: Vec4,
    color: Vec4,
    texcoord: Vec2,
    normal: Vec3,
}

impl VertexShaderOutput {
    pub fn new(
        world_pos: Vec4,
        window_pos: Vec4,
        color: Vec4,
        texcoord: Vec2,
        normal: Vec3,
    ) -> Self {
        Self {
            world_pos,
            window_pos,
            color,
            texcoord,
            normal,
        }
    }

    pub fn lerp(v1: VertexShaderOutput, v2: VertexShaderOutput, factor: f32) -> VertexShaderOutput {
        VertexShaderOutput::new(
            Vec4::lerp(v1.world_pos, v2.world_pos, factor),
            Vec4::lerp(v1.window_pos, v2.window_pos, factor),
            Vec4::lerp(v1.color, v2.color, factor),
            Vec2::lerp(v1.texcoord, v2.texcoord, factor),
            Vec3::lerp(v1.normal, v2.normal, factor)
        )
    }
}
