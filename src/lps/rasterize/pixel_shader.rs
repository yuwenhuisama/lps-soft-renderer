use super::pipeline::PixelShader;
use super::vt_output::VertexShaderOutput;
use crate::lps::common::math::vec4::Vec4;
use crate::lps::common::texture::Texture;
use std::sync::Mutex;
use std::{any::Any, sync::Arc};

pub struct CustomPixelShader {
    texture: Option<Arc<Mutex<Texture>>>,
}

impl CustomPixelShader {
    pub fn new() -> CustomPixelShader {
        CustomPixelShader { texture: None }
    }
}

impl PixelShader<VertexShaderOutput> for CustomPixelShader {
    fn handle(&self, pixel_fragment: &VertexShaderOutput) -> Vec4 {
        if let Some(texture) = self.texture.as_ref() {
            let texture = texture.lock().unwrap();
            let color = texture.sample2d(pixel_fragment.texcoord.clone());
            Vec4::new(color.r as f32, color.g as f32, color.b as f32, 255.0)
        } else {
            Vec4::new(
                pixel_fragment.color.x,
                pixel_fragment.color.y,
                pixel_fragment.color.z,
                255.0,
            )
        }
    }

    fn init_constant_buffer(&mut self, buffer: &Vec<Option<Arc<dyn Any + Send>>>) {
        self.texture = Some(
            buffer[3]
                .as_ref()
                .unwrap()
                .downcast_ref::<Arc<Mutex<Texture>>>()
                .unwrap()
                .clone(),
        );
    }
}
