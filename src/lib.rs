use std::path::PathBuf;
use slotmap::DefaultKey;

// === ФУНКЦИИ СКРИПТА ===
pub trait Script: Send {
	/// Выполняется один раз во время загрузки файла библиотеки
	fn init(&mut self, ctx: &dyn Context);
	/// Выполняется каждый main loop
	fn update(&mut self, ctx: &dyn Context, delta: f32);
}

// === МЕТОДЫ КОНТЕКСТА ===
pub trait Context {
	/// Доступ к подсистеме объектов
	fn objects(&self) -> &dyn ObjectServer;
	
	/// Логирование (можно вынести в отдельную подсистему позже)
	fn log(&self, msg: &str);
}

// === ПОДСИСТЕМА ОБЪЕКТОВ ===
pub trait ObjectServer {
	/// Создаёт корневой объект (без родителя)
	fn create_root(&self, object_name: String, script_path: Option<PathBuf>) -> DefaultKey;

	/// Создаёт дочерний объект
	/// Возвращает `None`, если родитель не существует
	fn create_child(&self, object_name: String, script_path: Option<PathBuf>, parent_object: DefaultKey) -> Option<DefaultKey>;

	/// Удаляет объект и всё его поддерево
	fn remove(&self, object_id: DefaultKey);

	///
	fn set_script(&self, object_id: DefaultKey, script_path: PathBuf) -> bool;

	///
	fn remove_script(&self, object_id: DefaultKey) -> bool;

	///
	fn move_to_parent(&self, childe_object_id: DefaultKey, new_parent_id: DefaultKey) -> bool;
}