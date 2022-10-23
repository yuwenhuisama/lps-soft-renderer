
pub enum RenderCommandType {
    BindVertexShader = 0,
    BindPixelShader = 1,
    SetVertexBuffer = 2,
    SetRenderTarget = 3,
}

pub trait RenderCommand {
    fn cmd_type(&self) -> RenderCommandType;
    fn execute(&self);
}
