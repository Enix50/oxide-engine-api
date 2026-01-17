use std::path::PathBuf;
/// Уникальный идентификатор объекта в сцене.
/// Гарантированно уникален в пределах одного запуска движка.
pub type EntityId = u64;

pub trait WorldContext {
    fn log(&self, msg: &str);

    //ObjectRegistry
    fn create_root(&self, name: String, script_path: Option<PathBuf>) -> EntityId;
    fn create_child(&self, name: String, script_path: Option<PathBuf>, parent: EntityId) -> Option<EntityId>;
    
}

pub trait Script: Send {
    fn init(&mut self, ctx: &dyn WorldContext);
    fn update(&mut self, ctx: &dyn WorldContext, delta: f32);
}