use std::path::PathBuf;
use crate::Object;

// === МЕТОДЫ КОНТЕКСТА ===
pub trait Context {
	/// Создаёт дочерний объект для указанного родителя
	/// object_name: имя для созданного объекта
	/// script_path: путь к файлу скрипта
	/// parent_object: объект к оторому будет добавлен объект
	/// Возвращает `None`, если родитель не существует или объект не может быть создан
	fn create_object(&mut self, object_name: String, script_path: Option<PathBuf>, parent_object: Option<&impl Object>) -> Option<&impl Object>;

	/// Удаляет объект и всё его поддерево
	fn remove_object(&mut self, object: &impl Object) -> bool;

	/// Устанавливает скрипт для объекта
	/// если script_path: None, скрипт будет удалён
	fn set_object_script(&mut self, object: &impl Object, script_path: Option<PathBuf>) -> bool;

	/// Перемещает объект к новому родителю
	/// если new_parent: None, объект станет корнем
	fn move_object_to_parent(&mut self, child_object: &impl Object, new_parent: Option<&impl Object>) -> bool;

	/// Получает объект по пути
	fn get_object(&self, object_path: &str) -> Option<&impl Object>;

	/// Добавляет объекту компонент
	fn add_component(&mut self, object: &impl Object, component: Box<impl crate::ComponentData>);

	/// Логирование (можно вынести в отдельную подсистему позже)
	fn log(&self, msg: &str);
}