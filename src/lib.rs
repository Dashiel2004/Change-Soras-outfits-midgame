#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]
mod sora;
mod config;
mod vars;
mod effect;
mod global;
#[skyline::main(name = "changing_outfit")]
pub fn main() {
    sora::install();
}
