pub trait Unit<'a> {
    fn init(&mut self);
    fn start(&mut self);
    fn exit(&mut self);
}
