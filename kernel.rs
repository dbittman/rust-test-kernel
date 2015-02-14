#![no_std]  //< Kernels can't use std
#![allow(unknown_features)]
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


#[macro_use]
mod macros;

// Prelude
mod prelude;
mod util;

/// Exception handling (panic)
pub mod unwind;

mod vga;

#[lang="start"]
#[no_mangle]
pub fn kmain()
{
    let mut d = vga::Display::new();
    d.clear();
    d.putch('A');
    let _ = write!(&mut d, "test");
    loop {}
}

