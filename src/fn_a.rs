pub use crate::struct_a1::A1;
pub use crate::struct_a2::A2;

pub fn global_fn_takes_a1_returns_a2(a1: A1)
    -> A2 {
    
    A2::new(&a1)
}
