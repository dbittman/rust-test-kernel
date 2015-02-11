#![no_std]  //< Kernels can't use std
#![allow(unknown_features)]
#![feature(lang_items)] //< unwind needs to define lang items
#![feature(asm)]    //< As a kernel, we need inline assembly
#![feature(core)]   //< libcore (see below) is not yet stablized


#[macro_use]
extern crate core;

use prelude::*;
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

/// Exception handling (panic)
pub mod unwind;

mod logging;

#[lang="start"]
#[no_mangle]
pub fn kmain()
{
    static mut VGA: *mut u16 = 0xb8000 as *mut u16;
    const COLOR: u16 = (0 << 12) | (15 << 8);
    unsafe {
        *VGA.offset(2 * 80 + 2) = 'A' as u16 | COLOR;
    }
    loop {}
}

