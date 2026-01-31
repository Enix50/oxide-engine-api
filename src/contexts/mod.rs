mod object_registry;
pub use object_registry::*;

// === МЕТОДЫ КОНТЕКСТА ===
pub trait Context {
	/// Доступ к подсистеме объектов
	fn get_object_registry(&self) -> &dyn ObjectRegistry;

	/// Логирование (можно вынести в отдельную подсистему позже)
	fn log(&self, msg: &str);
}