#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(
    unused_macros,
    unused_must_use,
    clippy::borrow_interior_mutable_const,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::absurd_extreme_comparisons,
    clippy::cmp_null,
    clippy::if_same_then_else
)]

mod glide;

#[skyline::main(name = "brawl_glide_port")]
pub fn main() {
    glide::install();
}