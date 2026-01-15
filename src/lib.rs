pub trait WorldContext {
    fn log(&self, msg: &str);
}

pub trait Script: Send {
    fn init(&mut self, ctx: &dyn WorldContext);
    fn update(&mut self, ctx: &dyn WorldContext, delta: f32);
}