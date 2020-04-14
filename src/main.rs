// All of the files that make up this module need to be listed in the root-file
// for the module. In the case of the crate module, this is "main.rs",
// so we list here all of the files that need to be accessible to any source files
// in this (the './src') folder

mod fn_a;           // Make all of the functions in fn_a.rs available to *all* files in this folder
mod struct_a1;      // Make struct A1 and all of its methods available
mod struct_a2;
mod sub_b;

pub use struct_a2::A2;  // To allow us to use 'A2' without a 'struct_a2::' prefix
 
fn main() {
    // 'A1' has no "pub use" statement, so we need to include the path to its module here ...
    let a1_created_here = struct_a1::A1::new();
    println!("struct_a1::a1_u32 = {}", a1_created_here.a1_u32);

    // Passing a1 to a global function defined in another module (file)
    let a2_returned_by_a1_method = fn_a::global_fn_takes_a1_returns_a2(a1_created_here);
    println!("struct_a2::a2_u32 = {}", a2_returned_by_a1_method.a2_u32);

    // Passing an instance of A1 into a method on A2 ...
    //
    // Because there is a "pub use" statement for A2 above, we do not need to include it when referring to A2, here
    let a2_created_here = A2::new(&struct_a1::A1{ a1_u32: 31 });
    println!("struct_a2::a2_u32 = {}", a2_created_here.a2_u32);

    // Now try to use the sub_b module
    sub_b::sub_module();
}
