// All of the files that make up this module need to be listed in the root-file
// for the module. In the case of the sub_b module, this is "sub_b.rs",
// so we list here all of the source files that make up the sub_b module and
// live in the sub_b folder ...
mod fn_b;
mod struct_b1;
mod struct_b2;

// "pub use" statements allow us to refer to other modules/files without having to
// specify the full path
pub use crate::struct_a1::A1;   // This comes from the root module
pub use struct_b1::B1;          // This is part of our module

// Export a global function that needs to use structs in this module and the root module, too
pub fn sub_module() {
    let struct_b1 = B1::new();  // Can use B1 without a path because statement 'pub use struct_b1::B1;' is included above
    println!("B1::b1_f64 = {}", struct_b1.b1_f64);

    let struct_a1 = A1::new();
    println!("A1::a1_u32 = {}", struct_a1.a1_u32);

    let struct_b2 = fn_b::global_fn_takes_a1_returns_b2(struct_a1);
    println!("B2::b2_f64 = {}", struct_b2.b2_f64);
}
