use slotmap::DefaultKey;

// === ТРЕЙТ ОБЪЕКТА ===
pub trait Object {
	/// Получить ID объекта
	fn get_id(&self) -> DefaultKey;

	/// Получить имя объекта
	fn get_name(&self) -> &str;

	/// Добавить дочерний объект
	fn add_child(&self, object_name: String, script_path: Option<std::path::PathBuf>) -> Option<&dyn Object>;

	/// Найти дочерний объект по имени
	fn find_child(&self, name: &str) -> Option<&dyn Object>;

	/// Получить родительский объект
	fn get_parent(&self) -> Option<&dyn Object>;

	/// Получить список дочерних объектов
	fn get_children(&self) -> Vec<&dyn Object>;
}