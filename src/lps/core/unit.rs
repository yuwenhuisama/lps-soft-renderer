pub trait Unit {
    fn init(&mut self);
    fn start(&mut self);
    fn exit(&mut self);
}
