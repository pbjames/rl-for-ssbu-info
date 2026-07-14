#![feature(proc_macro_hygiene)]
#![allow(
     unused_imports,
// 	unused_macros,
// 	unused_variables,
// 	unused_assignments,
// 	unused_unsafe,
// 	non_upper_case_globals,
// 	non_snake_case,
//     clippy::borrow_interior_mutable_const
)]

mod hooks;

#[skyline::main(name = "smash_cpu_info")]
pub fn main() {
    hooks::install();
}
