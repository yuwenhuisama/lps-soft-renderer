use super::math::{vec2::Vec2, vec3::Vec3, vec4::Vec4};

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
            color: Vec3::Zero,
            texcoord: Vec2::Zero,
            normal: Vec3::Zero,
        }
    }
}
