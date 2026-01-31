use std::path::PathBuf;
use crate::Object;

// === ПОДСИСТЕМА ОБЪЕКТОВ ===
pub trait ObjectRegistry {
	/// Создаёт дочерний объект для указанного родителя
	/// object_name: имя для созданного объекта
	/// script_path: путь к файлу скрипта
	/// parent_object: объект к оторому будет добавлен объект
	/// Возвращает `None`, если родитель не существует или объект не может быть создан
	fn create_child(&self, object_name: String, script_path: Option<PathBuf>, parent_object: Option<&dyn Object>) -> Option<&dyn Object>;

	/// Удаляет объект и всё его поддерево
	fn remove(&self, object: &dyn Object) -> bool;

	/// Устанавливает скрипт для объекта
	fn set_script(&self, object: &dyn Object, script_path: PathBuf) -> bool;

	/// Убирает скрипт с объекта
	fn remove_script(&self, object: &dyn Object) -> bool;

	/// Перемещает объект к новому родителю
	fn move_to_parent(&self, child_object: &dyn Object, new_parent: &dyn Object) -> bool;

	/// Получает объект по пути
	fn get(&self, object_path: &str) -> Option<&dyn Object>;

	/// Добавляет объекту компонент
	fn add_component(&self, object: &dyn Object, component: Box<dyn crate::ComponentData>);
}