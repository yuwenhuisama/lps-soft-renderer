use crate::lps::common::math::{vec2::Vec2, vec3::Vec3, vec4::Vec4};

#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pub position: Vec4,
    pub color: Vec3,
    pub texcoord: Vec2,
    pub normal: Vec3,
}

impl Vertex {
    pub fn new(position: Vec4, color: Vec3, texcoord: Vec2, normal: Vec3) -> Self {
        Self {
            position,
            color,
            texcoord,
            normal,
        }
    }

    pub fn new_with_pos(position: Vec4) -> Self {
        Self {
            position,
            color: Vec3::ZERO,
            texcoord: Vec2::ZERO,
            normal: Vec3::ZERO,
        }
    }
}

pub type VertexShaderInput = Vertex;
