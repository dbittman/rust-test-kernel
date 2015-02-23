use io::{outportb,inportb};

/* This structure is what is on the stack at the time the interrupt is received by the
 * rust code. This is accomplished through some magic x86 calling conventions */
#[derive(Copy)]
#[repr(C, packed)]
pub struct Registers {
    ds: u32,
    edi: u32, esi: u32, ebp: u32, esp: u32, ebx: u32, edx: u32, ecx: u32, eax: u32,
    int_num: u32, err_code: u32,
    eip: u32, cs: u32, eflags: u32, useresp: u32, ss: u32
}

fn interrupt_unhandled(regs: Registers) {
    print!("unhandled interrupt: {}\n", regs.int_num);
}

static mut interrupt_table: [fn(Registers); 16] = [interrupt_unhandled; 16];

/* General interrupt handler. If it's >= 32, then it's an external interrupt (like
 * the keyboard). If it isn't, then it's an exception (like divide by zero) */
#[no_mangle] //need no_mangle because this is called from assembly code
pub unsafe extern "C" fn interrupt_handler(regs: Registers)
{
    if regs.int_num >= 32 {
        let irq: usize = regs.int_num as usize - 32;
        interrupt_table[irq](regs);
        if regs.int_num >= 40 {
            outportb(0xA0, 0x20);
        }
        outportb(0x20, 0x20);
    } else {
        print!("EXCEPTION {}", regs.int_num);
        asm!("hlt");
    }
}

pub fn register_handler(index: usize, handler: fn(Registers))
{
    unsafe {
        interrupt_table[index] = handler;
    }
}

pub fn sti()
{
    unsafe{ asm!("sti") };
}

pub fn cli()
{
    unsafe{ asm!("cli") };
}

