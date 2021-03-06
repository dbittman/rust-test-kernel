use io::{outportb,inportb};
use interrupt::*;
use vga::*;
static mut ticks: usize = 0;

#[allow(unused_variables)]
pub fn timer_tick(regs: Registers)
{
    unsafe {
        ticks += 1;
    if ticks % 100 == 0 {
        print!("tick");
    }
    }
}

pub fn init()
{
    ::interrupt::register_handler(0, timer_tick);
}

