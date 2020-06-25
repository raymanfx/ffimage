pub mod generic;
pub use generic::{GenericBuffer, GenericFlatBuffer, GenericView};

pub mod dynamic;
pub use dynamic::StorageType as DynamicStorageType;
pub use dynamic::{DynamicBuffer, DynamicView};
