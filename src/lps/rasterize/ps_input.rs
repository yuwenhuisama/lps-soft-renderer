use crate::lps::common::math::{vec2::Vec2, vec3::Vec3, vec4::Vec4};

pub struct PixelShaderInput {
    pub color: Vec4,
}

impl PixelShaderInput {
    pub fn new(color: Vec4) -> Self {
        Self {
            color,
        }
    }
}
