use super::pipeline::PixelShader;

pub struct CustomPixelShader {
}

impl PixelShader<PixelShaderInput, PixelShaderOutput> for CustomPixelShader {
    fn handle(&self, pixel_fragment: &PixelShaderInput) -> PixelShaderOutput {
        todo!()
    }
}
