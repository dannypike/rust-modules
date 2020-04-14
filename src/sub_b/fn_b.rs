// we are in submodule sub_b, so we need to use absolute paths to import things from the main module
pub use crate::struct_a1::A1;

// But we can use relative paths to import other structs from this module
pub use super::struct_b2::B2;

pub fn global_fn_takes_a1_returns_b2(a1: A1) -> B2 {
    B2::new(&a1)
}
