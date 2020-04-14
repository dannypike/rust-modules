pub use crate::struct_a1::A1;

pub struct B2 {
    pub b2_f64: f64,
}

impl B2 {
    pub fn new(a1: &A1) -> B2 {
        B2 {
            b2_f64: a1.a1_u32.into(),
        }
    }
}
