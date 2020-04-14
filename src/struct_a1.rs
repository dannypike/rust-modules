pub use crate::struct_a2::A2;

pub struct A1 {
    pub a1_u32: u32,
}

impl A1 {
    pub fn new() -> A1 {
        A1 {
            a1_u32: 42,
        }
    }
}
