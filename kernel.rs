#![no_std]  //< Kernels can't use std
#![allow(unstable)]
#![allow(unused_imports)]
#![allow(unknown_features)]
#![allow(dead_code)]
#![feature(lang_items)] //< unwind needs to define lang items
#![feature(asm)]    //< As a kernel, we need inline assembly
#![feature(core)]   //< libcore (see below) is not yet stablized
#[macro_use]
extern crate core;

use prelude::*;
use core::fmt::Writer;
mod std {
    // #18491 - write!() expands to std::fmt::Arguments::new
    pub use core::fmt;
    // #16803 - #[derive] references std::cmp
    pub use core::cmp;
    // ??? - For loops reference std
    pub use core::iter;
    pub use core::option;
    // ??? - Derive references marker/ops
    pub use core::marker;
}

// Prelude
mod prelude;
mod util;


#[macro_use]
mod vga;
/// Exception handling (panic)
pub mod unwind;
pub mod x86_tables;

#[lang="start"]
#[no_mangle]
pub fn kmain()
{
    let d = vga::Display::new();
    d.clear();

    print!("Hello World");
    
    unsafe {
    x86_tables::cli();
    x86_tables::gdt_init();
    x86_tables::idt_init();
    x86_tables::pic_init();
    x86_tables::sti();
    }
    loop {}
}

