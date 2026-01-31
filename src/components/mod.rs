// Макрос для автоматической реализации ComponentData
macro_rules! impl_component_data {
    ($struct_name:ty) => {
        impl crate::components::ComponentData for $struct_name {
            fn get_type_name(&self) -> &str {
                stringify!($struct_name)
            }
        }
    };
}

mod transform;
pub use transform::*;

pub trait ComponentData: Send + Sync {
    fn get_type_name(&self) -> &str;
}