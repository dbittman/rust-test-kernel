use io::{outportb,inportb};
#[derive(Copy)]
#[repr(C, packed)]
pub struct Registers {
    ds: u32,
    edi: u32, esi: u32, ebp: u32, esp: u32, ebx: u32, edx: u32, ecx: u32, eax: u32,
    int_num: u32, err_code: u32,
    eip: u32, cs: u32, eflags: u32, useresp: u32, ss: u32
}

static mut ints: u32 = 0;
fn interrupt_unhandled(regs: Registers) {
    print!("unhandled interrupt: {}\n", regs.int_num);
}

static mut interrupt_table: [fn(Registers); 16] = [interrupt_unhandled; 16];

#[no_mangle]
pub unsafe extern "C" fn interrupt_handler(regs: Registers)
{
    ints += 1;
    if regs.int_num >= 32 {
        let irq: usize = regs.int_num as usize - 32;
        interrupt_table[irq](regs);
        if regs.int_num >= 40 {
            outportb(0xA0, 0x20);
        }
        outportb(0x20, 0x20);
    } else {
        print!("EXCEPTION");
        asm!("hlt");
    }
}

pub fn interrupt_register_handler(index: usize, handler: fn(Registers))
{
    unsafe {
        interrupt_table[index] = handler;
    }
}

pub unsafe fn sti()
{
    asm!("sti");
}

pub unsafe fn cli()
{
    asm!("cli");
}

