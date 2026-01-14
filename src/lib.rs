#[no_mangle]
pub extern "Rust" fn create_script() -> Box<dyn Script> {
    panic!("Override in script!");
}

pub trait WorldContext {
    fn log(&self, msg: &str);
}

pub trait Script {
    fn init(&mut self, ctx: &dyn WorldContext);
}