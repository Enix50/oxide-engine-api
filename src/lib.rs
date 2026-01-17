use std::path::PathBuf;

/// Уникальный идентификатор объекта в сцене.
pub type EntityId = u64;

// === Подсистема объектов ===
pub trait ObjectSystem {
	/// Создаёт корневой объект (без родителя).
	fn create_root(&self, name: String, script_path: Option<PathBuf>) -> EntityId;

	/// Создаёт дочерний объект.
	/// Возвращает `None`, если родитель не существует.
	fn create_child(&self, name: String, script_path: Option<PathBuf>, parent: EntityId) -> Option<EntityId>;

	/// Удаляет объект и всё его поддерево.
	fn remove(&self, id: EntityId);
}

// === Основной контекст движка ===
pub trait WorldContext {
	/// Доступ к системе объектов.
	fn objects(&self) -> &dyn ObjectSystem;
	
	/// Логирование (можно вынести в отдельную подсистему позже)
	fn log(&self, msg: &str);
}

pub trait Script: Send {
	fn init(&mut self, ctx: &dyn WorldContext);
	fn update(&mut self, ctx: &dyn WorldContext, delta: f32);
}