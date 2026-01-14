pub trait WorldContext {
    fn log(&self, msg: &str);
}

pub trait Script {
    fn init(&mut self, ctx: &dyn WorldContext);
}