#![no_std]
#![allow(unstable)]
#![allow(unused_imports)]
#![allow(unknown_features)]
#![allow(dead_code)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(core)]
#![allow(unused_unsafe)]
#![feature(box_syntax)]
#![no_builtins]
#[macro_use]
extern crate core;

use prelude::*;
use core::fmt::Writer;
mod std {
    pub use core::fmt;
    pub use core::cmp;
    pub use core::iter;
    pub use core::option;
    pub use core::marker;
}

// Prelude
mod prelude;
pub mod util;
mod allocator;
mod boxed;

#[macro_use]
mod vga;
mod unwind;
mod io;
mod x86;

// This needs to be public because it exports a function that gets called by assembly code
pub mod interrupt;

mod keyboard;
mod timer;

#[lang="start"]
#[no_mangle]
pub fn kmain()
{
    /* make sure we've got interrupts disabled while we set things up */
    interrupt::cli();
    /* initialize the memory allocator */
    allocator::init();

    /* clear the screen and print a message */
    vga::Display::new().clear();
    print!("Hello World from Rust!\n");
    /* TODO:
     * Interface with Go?
     * More OS features?
     * Fix unwind code...make panic! work
     * Implement x86 exceptions
     * Cleanup and comments
     */

    /* initialize CPU things (GDT, IDT, etc) */
    x86::initialize_processor();
    timer::init();
    keyboard::init();
    
    /* and start firing off interrupts */
    print!("Done booting up!\n");
//    loop {}
    interrupt::sti();

    /* loop forever, handling interrupts */
    loop {}
}

