pub use std::path::PathBuf;
pub use slotmap::DefaultKey;

// === СОЗДАНИЕ СКРИПТА ===
#[macro_export]
macro_rules! create_script {
    ($struct_name:ident { $($field:ident: $value:expr),* $(,)? }) => {
        #[unsafe(no_mangle)]
        pub extern "Rust" fn create_script() -> Box<dyn Script> {
            Box::new($struct_name {
            	$($field: $value),*
        	})
        }
    };
}

// === События ===
pub enum Event {
	Key{ code: u32 }
}

// === ФУНКЦИИ СКРИПТА ===
pub trait Script: Send {
	/// Выполняется один раз во время загрузки файла библиотеки
	fn init(&mut self, ctx: &impl Context) {
		println!("Script Init");
	}
	/// Выполняется когда ---
	fn ready(&mut self, ctx: &impl Context) {
		println!("Script Ready");
	}
	/// Выполняется каждый main loop
	fn update(&mut self, ctx: &impl Context, delta: f32) {
		println!("Script Update");
	}
	/// Вызывается при вводе
	fn event(&mut self, ctx: &impl Context, event: Event) {
		println!("Script Event");
	}
}

// === МЕТОДЫ КОНТЕКСТА ===
pub trait Context {
	/// Доступ к подсистеме объектов
	fn objects(&self) -> &impl ObjectServer;

	/// Логирование (можно вынести в отдельную подсистему позже)
	fn log(&self, msg: &str);
}

// === ПОДСИСТЕМА ОБЪЕКТОВ ===
pub trait ObjectServer {
	/// Создаёт корневой объект
	/// Возвращает `None`, если родитель не существует
	fn create_root(&self, object_name: String, script_path: Option<PathBuf>) -> Option<DefaultKey>;

	/// Создаёт дочерний объект для себя
	/// Возвращает `None`, если родитель не существует
	fn create_child(&self, object_name: String, script_path: Option<PathBuf>) -> Option<DefaultKey>;

	/// Создаёт дочерний объект для parent_object
	/// Возвращает `None`, если родитель не существует
	fn create_child_for(&self, object_name: String, script_path: Option<PathBuf>, parent_object: DefaultKey) -> Option<DefaultKey>;

	/// Удаляет объект и всё его поддерево
	fn remove(&self, object_id: DefaultKey) -> bool;

	///
	fn set_script(&self, object_id: DefaultKey, script_path: PathBuf) -> bool;

	///
	fn remove_script(&self, object_id: DefaultKey) -> bool;

	///
	fn move_to_parent(&self, child_object_id: DefaultKey, new_parent_id: DefaultKey) -> bool;

	fn get_by_id(&self, object_id: DefaultKey) -> Option<DefaultKey>;

	fn get_by_name(&self, object_name: PathBuf) -> Option<DefaultKey>;

	fn test_create_root(&self, object_name: String, script_path: Option<PathBuf>) -> Option<&impl Object>;
}

pub trait Object {
	fn add_child(&self, object_name: String, script_path: Option<PathBuf>) -> Option<&impl Object>;
}