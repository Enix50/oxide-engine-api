use std::path::PathBuf;
use crate::Object;

// === МЕТОДЫ КОНТЕКСТА ===
pub trait Context {
	/// Создаёт дочерний объект для указанного родителя
	/// object_name: имя для созданного объекта
	/// script_path: путь к файлу скрипта
	/// parent_object: объект к оторому будет добавлен объект
	/// Возвращает `None`, если родитель не существует или объект не может быть создан
	fn create_object(&mut self, object_name: String, script_path: Option<PathBuf>, parent_object: Option<&dyn Object>) -> Option<&dyn Object>;

	/// Удаляет объект и всё его поддерево
	fn remove_object(&mut self, object: &dyn Object) -> bool;

	/// Устанавливает скрипт для объекта
	/// если script_path: None, скрипт будет удалён
	fn set_object_script(&mut self, object: &dyn Object, script_path: Option<PathBuf>) -> bool;

	/// Перемещает объект к новому родителю
	/// если new_parent: None, объект станет корнем
	fn move_object_to_parent(&mut self, child_object: &dyn Object, new_parent: Option<&dyn Object>) -> bool;

	/// Получает объект по пути
	fn get_object(&self, object_path: &str) -> Option<&dyn Object>;

	/// Добавляет объекту компонент
	fn add_component(&mut self, object: &dyn Object, component: Box<dyn crate::ComponentData>);

	/// Логирование (можно вынести в отдельную подсистему позже)
	fn log(&self, msg: &str);
}