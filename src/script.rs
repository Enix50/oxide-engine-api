use crate::Context;

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

// === ФУНКЦИИ СКРИПТА ===
pub trait Script: Send {
	/// Выполняется при загрузке файла библиотеки
	fn init(&mut self, ctx: &dyn Context) {
		ctx.log("Script Init");
	}
	/// Выполняется при добавлении в древо регистра
	fn ready(&mut self, ctx: &dyn Context) {
		ctx.log("Script Ready");
	}
	/// Выполняется каждый main loop
	fn update(&mut self, ctx: &dyn Context, delta: f32) {
		ctx.log("Script Update");
		println!("{}",delta);
	}
	/// Вызывается при вводе
	fn event(&mut self, ctx: &dyn Context, event: crate::Event) {
		ctx.log("Script Event");
	}
}