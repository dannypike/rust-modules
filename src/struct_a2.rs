pub use crate::struct_a1::A1;

pub struct A2 {
    pub a2_u32: u32,
}

impl A2 {
    pub fn new(a1: &A1) -> A2 {
        A2 {
            a2_u32: a1.a1_u32,
        }
    }
}
